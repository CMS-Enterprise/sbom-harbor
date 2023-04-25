use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;

use crate::auth::{Action, Authorizer, Effect, Group, Policy, Resource, Role, User};
use crate::mongodb::Store;
use crate::Error;

/// Allows quick categorization of an assertion effect. Useful for exiting early when explicit
/// deny-all or allow-all policies apply.
pub enum Profile {
    /// Profile is denied access to all resources.
    DenyAll,
    /// Profile is allowed access to any resource.
    AllowAny,
    /// Profile requires policy evaluation for the given resource.
    Default,
}

/// Convention-based Authorizer implementation for MongoDB document access.
#[derive(Clone, Debug)]
pub struct DefaultAuthorizer {
    store: Arc<Store>,
}

#[async_trait]
impl Authorizer for DefaultAuthorizer {
    #[instrument]
    async fn assert(
        &self,
        user: User,
        resource: Resource,
        action: Action,
    ) -> Result<Effect, Error> {
        let groups = self.groups_by_user(user).await?;
        let roles = self.roles_by_user_groups(groups).await?;
        let policies = self.policies_by_resource(resource).await?;
        let profile = profile(&roles, &policies);

        match profile {
            Profile::DenyAll => Ok(Effect::Deny),
            Profile::AllowAny => Ok(Effect::Allow),
            Profile::Default => assert_role_policy(roles, policies, action),
        }
    }
}

impl DefaultAuthorizer {
    /// Factory method for creating a new [DefaultAuthorizer] instance.
    pub fn new(store: Arc<Store>) -> DefaultAuthorizer {
        DefaultAuthorizer { store }
    }

    /// List the [Groups] the [User] is a member of.
    async fn groups_by_user(&self, user: User) -> Result<Vec<Group>, Error> {
        let filter = HashMap::from([("users", user.id.as_str())]);
        self.store.query::<Group>(filter).await
    }

    /// List the policies that apply to a [Resource].
    async fn policies_by_resource(&self, resource: Resource) -> Result<Vec<Policy>, Error> {
        let filter = HashMap::from([("resource_id", resource.id.as_str())]);
        self.store.query::<Policy>(filter).await
    }

    /// List the [Roles] associated with a set of [Groups].
    async fn roles_by_user_groups(&self, groups: Vec<Group>) -> Result<Vec<Role>, Error> {
        let mut filter = HashMap::new();

        groups.iter().for_each(|group| {
            group.roles.iter().for_each(|role_id| {
                filter.insert("id", role_id.as_str());
            });
        });

        self.store.query::<Role>(filter).await
    }
}

// TODO: Implement short circuit for users that are disabled or are super users
/// Categorize the profile of the assertion based on a set of [Roles] and [Policies].
pub fn profile(_roles: &[Role], _policies: &[Policy]) -> Profile {
    // let profile = Profile::Default;

    // TODO: This might require a stateful struct do avoid an inflexible convention around deny/allow all.
    // roles
    //     .into_iter()
    //     .for_each(|role| {
    //         if role.policies
    //     });

    Profile::Default
}

/// Calculates the authorization [Effect] for an [Action].
pub fn assert_role_policy(
    roles: Vec<Role>,
    policies: Vec<Policy>,
    action: Action,
) -> Result<Effect, Error> {
    let policies = policies_for_roles(policies, roles);
    println!("policies for roles: {:#?}", policies);

    // Check for explicit deny. An explicit deny overrides an explicit allow.
    if explicitly_denied(&policies) {
        return Ok(Effect::Deny);
    }

    // Check for explicit allow. An explicit allow is required.
    if explicitly_allowed(&policies, action) {
        return Ok(Effect::Allow);
    }

    // Deny by default
    Ok(Effect::Deny)
}

// TODO: Refactor these filters to finds.
/// Asserts whether a pre-filtered set of [Policies] includes an explicit deny.
pub fn explicitly_denied(policies: &[Policy]) -> bool {
    !policies
        .iter()
        .filter(|policy| policy.effect == Effect::Deny)
        .collect::<Vec<&Policy>>()
        .is_empty()
}

/// Asserts whether a pre-filtered set of [Policies] includes an explicit allow.
pub fn explicitly_allowed(policies: &[Policy], action: Action) -> bool {
    !policies
        .iter()
        .filter(|policy| policy.effect == Effect::Allow && policy.action == action)
        .collect::<Vec<&Policy>>()
        .is_empty()
}

/// Filter policies for a resource that are referenced by a set of [Roles].
pub fn policies_for_roles(policies: Vec<Policy>, roles: Vec<Role>) -> Vec<Policy> {
    policies
        .into_iter()
        .filter(|policy| {
            // this filter determines whether this role pertains to this policy.
            roles.iter().any(|role| role.policies.contains(&policy.id))
        })
        .collect()
}
