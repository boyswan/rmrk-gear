use crate::*;

impl From<MultiResource> for MultiResourceState {
    fn from(multiresource: MultiResource) -> MultiResourceState {
        MultiResourceState {
            pending_resources: multiresource
                .pending_resources
                .iter()
                .map(|(key, value)| (*key, value.iter().copied().collect()))
                .collect(),
            active_resources: multiresource
                .active_resources
                .iter()
                .map(|(key, value)| (*key, value.iter().copied().collect()))
                .collect(),
            resource_overwrites: multiresource
                .resource_overwrites
                .iter()
                .map(|(key, value)| {
                    (
                        *key,
                        value
                            .clone()
                            .iter()
                            .map(|(key, value)| (*key, *value))
                            .collect(),
                    )
                })
                .collect(),
            active_resources_priorities: multiresource
                .active_resources_priorities
                .iter()
                .map(|(key, value)| (*key, value.clone()))
                .collect(),
        }
    }
}

impl From<&RMRKToken> for RMRKState {
    fn from(rmrk: &RMRKToken) -> RMRKState {
        RMRKState {
            name: rmrk.name.clone(),
            symbol: rmrk.symbol.clone(),
            admin: rmrk.admin,
            pending_children: rmrk
                .nesting
                .pending_children
                .iter()
                .map(|(key, value)| (*key, value.iter().copied().collect()))
                .collect(),
            accepted_children: rmrk
                .nesting
                .accepted_children
                .iter()
                .map(|(key, value)| (*key, value.iter().copied().collect()))
                .collect(),
            children_status: rmrk
                .nesting
                .children_status
                .iter()
                .map(|(key, value)| (*key, *value))
                .collect(),
            multiresource: rmrk.multiresource.clone().into(),
            resource_id: rmrk.resource_id,
            equipped_tokens: rmrk.equipped_tokens.iter().copied().collect(),
        }
    }
}
