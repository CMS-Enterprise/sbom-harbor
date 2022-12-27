const stringify = (obj) => {
    return JSON.stringify(obj, null, 4);
}

const debug = true;
const log = (message) => {
    if (!debug) {
        return
    }

    console.log(message);
}

const log_req = (url, body) => {
    log(`req: url - ${url} - body: ${stringify(body)}`);
}

const log_resp = (url, response) => {
    log(`response: url - ${url} body - ${stringify(response)}`);
}

export class TestContext {
    constructor() {
        this.testId = uuidv4();
        this.teamIds = [];
        this.username = `${__ENV.ADMIN_USERNAME}`;
        this.password = `${__ENV.ADMIN_PASSWORD}`;
        // Create an enabled token, but set it to expire immediately so that it can't ever be used.
        this.expiryDate = new Date();
    }

    forLog() {
        return (({ testId, teamName, team, teamIds, expiryDate }) => ({ testId, teamName, team, teamIds, expiryDate }))(this);
    }

    login() {
        let url = BASE_URL + `/api/v1/login`;
        let params = {headers: {"Content-Type": "application/json", "Accept": "application/json"}};

        // log_req(url, body);
        let response = http.post(url, JSON.stringify(this.bodyFor("login")), params);
        // log_resp(url, response.body);

        if (response.status !== 200) {
            this.jwt = undefined;
            return false;
        }

        this.jwt = JSON.parse(response.body).token;
        // log(`login success: ${this.jwt}`);

        this.headerParams = {
            headers: {
                "Content-Type": "application/json", "Accept": "application/json",
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
        this.teamIds.forEach(teamId => {
            let url = BASE_URL + `/api/v1/team/${teamId}?children=true`;
            let response = http.del(url, this.headerParams);

            if (response.status !== 200) {
                log(`TestContext.teardown failed for team id ${teamId} with ${response.status} - ${response.body}`)
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
            case " codebaseId":
                this.ensureCodebase();
                return this.codebase.id;
            case "tokenId":
                this.ensureToken();
                return this.token.id;
        }
    }

    bodyFor(entityName) {
        switch (entityName) {
            case "login":
                return {username: this.username, password: this.password};
            case "team":
                // TODO: Add unique constraint on name and ensure it's enforced.
                return {id: "", name: this.teamName, members: [], projects: []};
            case "member":
                return {email: this.username, isTeamLead: true};
            case "project":
                return {id: "", name: this.entityName("project"), fisma: "", codebases: []};
            case "codebase":
                return {id: "", name: this.entityName("codebase"), language: "", buildTool: ""};
            case "token":
                return {name: this.entityName("token"), created: "", enabled: true, expires: this.expiryDate.toISOString(), token: this.testId};
            default:
                return undefined;
        }
    }

    createEntity(url, body, funcName) {
        // log_req(url, body);
        let response = http.post(url, JSON.stringify(body), this.headerParams);
        // log_resp(url, response.json());

        if (response.status !== 200) {
            console.error(`${funcName} failed with status ${response.status} and message ${response.body}`);
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

        this.team = this.createEntity(url, this.bodyFor("team"), "ensureTeam");
        this.teamIds.push(this.team.id);
    }

    ensureMember() {
        if (this.member) {
            return
        }

        // Ensure upstream requirements
        this.ensureTeam();

        let url = BASE_URL + `/api/v1/member?teamId=${this.team.id}`;

        this.member = this.createEntity(url, this.bodyFor("member"), "ensureMember");
    }

    ensureProject() {
        if (this.project) {
            return
        }

        // Ensure upstream requirements
        this.ensureTeam();

        // TODO: Dynamically test both values of booleans like children.
        let url = BASE_URL + `/api/v1/project?teamId=${this.team.id}&children=true`;

        this.project = this.createEntity(url, this.bodyFor("project"), "ensureProject");
    }

    ensureCodebase() {
        if (this.codebase) {
            return
        }

        this.ensureTeam();
        this.ensureProject();

        let url = BASE_URL + `/api/v1/codebase?teamId=${this.team.id}&projectId=${this.project.id}`;

        this.codebase = this.createEntity(url, this.bodyFor("codebase"), "ensureCodebase");
    }

    ensureToken() {
        if (this.token) {
            return
        }

        let url = BASE_URL + `/api/v1/token?teamId=${this.team.id}`;

        this.token = this.createEntity(url, this.bodyFor("token"), "ensureToken");
    }
}

export const initContext = () => {
    const ctx = new TestContext()

    if (!ctx.buildUp()) {
        console.error("failed to build TestContext");
        return;
    }

    log(`initiating e2e tests with ctx: ${stringify(ctx.forLog())}`);

    return ctx;
}
