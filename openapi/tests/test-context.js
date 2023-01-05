import http from "k6/http";
import { uuidv4 } from 'https://jslib.k6.io/k6-utils/1.4.0/index.js';

const BASE_URL = `${__ENV.CF_DOMAIN}`

// Utility method to reduce boilerplate. Pretty prints JSON output.
const stringify = (obj) => {
    return JSON.stringify(obj, null, 4);
}

let debug = false;
const log = (message) => {
    if (!debug) {
        return
    }

    console.log(message);
}

// Debug helper method to log output during development.
const log_req = (url, body) => {
    if (body) {
        log(`req: url - ${url} - body: ${stringify(body)}`);
        return;
    }

    log(`req: url - ${url}`);
}

// Debug helper method to log output during development.
const log_resp = (url, response) => {
    log(`response: url - ${url} body - ${stringify(response)}`);
}

export class TestContext {
    constructor() {
        this.testId = uuidv4();
        this.teamIds = [];
        this.username = `${__ENV.ADMIN_USERNAME}`;
        this.password = `${__ENV.ADMIN_PASSWORD}`;
        this.sbom = open("../sbom-fixture.json");

        const date = new Date();
        let expiry = new Date(date.setTime(date.getTime() + 1 * 60 * 60 * 1000));
        this.expiryDate = expiry.toISOString();
    }

    init() {
        if (!this.buildUp()) {
            console.error("failed to build TestContext");
            return false;
        }

        console.log(`login succeeded: proceeding with tests for run ${this.testId}`);

        return true;
    }

    login() {
        let url = BASE_URL + `/api/v1/login`;
        let params = {headers: {"Content-Type": "application/json", "Accept": "application/json"}};

        let body = this.bodyFor("/api/v1/login");

        // log_req(url, body);
        let response = http.post(url, body, params);
        // log_resp(url, response.body);

        if (response.status !== 200) {
            this.jwt = undefined;
            console.log(`failed login - status: ${response.status} for body ${body}`);
            return false;
        }

        this.jwt = JSON.parse(response.body).token;

        this.headerParams = {
            headers: {
                "Content-Type": "application/json",
                "Accept": "application/json",
                "Authorization": `${this.jwt}`
            }
        };

        return true;
    }

    entityName(entityType) {
        if (!entityType) {
            entityType = "unknown";
        }

        return `e2e-${entityType}-${this.testId}`;
    }

    buildUp() {
        if (!this.login()) {
            return false;
        }

        this.teamName = this.entityName("team");

        this.ensureTeam();

        if (!this.team) {
            log(`TestContext.buildUp failed to create team: ${stringify(this)}`)
            return false;
        }

        if (this.team.name != this.teamName) {
            log(`TestContext.buildUp failed name check: ${stringify(this)}`)
            return false;
        }

        return true;
    }

    teardown() {
        // TODO: Validate these all work once the API KeyError on team deletion is fixed.
        this.teamIds.forEach(teamId => {
            let url = BASE_URL + `/api/v1/team/${teamId}?children=true`;
            console.log(`tearing down team ${teamId}`);

            let response = http.del(url, {}, this.headerParams);

            if (response.status !== 200) {
                console.log(`teardown failed for team id ${teamId} with ${response.status} - ${response.body}`);
            } else {
                console.log(`teardown complete for team id ${teamId} with ${response.status} - ${response.body}`);
            }
        });
    }

    getParamValue(queryStringKey) {
        switch(queryStringKey) {
            case "children":
                return true;
            case "filter":
                return this.username;
            case "teamId":
                this.ensureTeam();
                return this.team.id;
            case "memberId":
                this.ensureMember();
                return this.member.id;
            case "projectId":
                this.ensureProject();
                return this.project.id;
            case "codebaseId":
                this.ensureCodebase();
                return this.codebase.id;
            case "tokenId":
                this.ensureToken();
                return this.token.id;
        }
    }

    bodyFor(route) {
        switch (route) {
            case "/api/v1/login":
                return stringify({username: this.username, password: this.password});
            case "/api/v1/team":
                return stringify({id: "", name: this.teamName, members: [], projects: []});
            case "/api/v1/member":
                return stringify({email: this.username, isTeamLead: true});
            case "/api/v1/project":
                return stringify({id: "", name: this.entityName("project"), fisma: "", codebases: []});
            case "/api/v1/codebase":
                return stringify({id: "", name: this.entityName("codebase"), language: "", buildTool: "", cloneUrl: "https://github.com/cmsgov/ab2d-lambdas"});
            case "/api/v1/token":
                return stringify({name: this.entityName("token"), created: "", enabled: true, expires: this.expiryDate});
            case "/api/v1/{teamId}/{projectId}/{codebaseId}/sbom":
                return this.sbom;
            default:
                return stringify({});
        }
    }

    tokenFor(route) {
        switch (route) {
            case "/api/v1/{teamId}/{projectId}/{codebaseId}/sbom":
                this.token = undefined;
                this.ensureToken();
                return this.token.token;
            default:
                return this.jwt;
        }
    }

    createEntity(url, body, funcName) {
        let response = http.post(url, body, this.headerParams);

        if (response.status !== 200) {
            console.error(`${funcName} failed with: ${stringify(response.body)}`);
        }

        let result = JSON.parse(response.body);

        // log(`TestContext.${funcName}: ${stringify(result)}`);

        return result;
    }

    ensureTeam() {
        if (this.team) {
            return
        }

        let url = BASE_URL + `/api/v1/team?children=true`;

        this.team = this.createEntity(url, this.bodyFor("/api/v1/team"), "ensureTeam");
        this.teamIds.push(this.team.id);
    }

    ensureMember() {
        if (this.member) {
            return
        }

        // Ensure upstream requirements
        this.ensureTeam();

        let url = BASE_URL + `/api/v1/member?teamId=${this.team.id}`;

        this.member = this.createEntity(url, this.bodyFor("/api/v1/member"), "ensureMember");
    }

    ensureProject() {
        if (this.project) {
            return
        }

        // Ensure upstream requirements
        this.ensureTeam();

        // TODO: Dynamically test both values of booleans like children.
        let url = BASE_URL + `/api/v1/project?teamId=${this.team.id}&children=true`;

        this.project = this.createEntity(url, this.bodyFor("/api/v1/project"), "ensureProject");
    }

    ensureCodebase() {
        if (this.codebase) {
            return
        }

        // Ensure upstream requirements
        this.ensureTeam();
        this.ensureProject();

        let url = BASE_URL + `/api/v1/codebase?teamId=${this.team.id}&projectId=${this.project.id}`;

        this.codebase = this.createEntity(url, this.bodyFor("/api/v1/codebase"), "ensureCodebase");
    }

    ensureToken() {
        if (this.token) {
            return
        }

        let url = BASE_URL + `/api/v1/token?teamId=${this.team.id}`;

        this.token = this.createEntity(url, this.bodyFor("/api/v1/token"), "ensureToken");
    }

    isFixture(method, route) {
        if (method === "del" && route.includes("team")) {
            console.log(`deferring fixture delete to teardown: ${method} - ${route}`);

            return true;
        }

        return false;
    }

    handleFixtures(method, route, responseBody) {
        if (method !== "post" || route !== "/api/v1/team") {
            return;
        }

        console.log(`adding fixture to teardown: ${method} - ${route} - ${responseBody}`);
        let body = JSON.parse(responseBody);
        this.teamIds.push(body.id);
    }
}
