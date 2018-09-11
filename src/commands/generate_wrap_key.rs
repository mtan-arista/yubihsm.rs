//! Generate a wrapping (i.e. encryption) key within the `YubiHSM2`
//!
//! <https://developers.yubico.com/YubiHSM2/Commands/Generate_Wrap_Key.html>

use super::generate_key::GenerateKeyParams;
use super::{Command, Response};
use {
    Adapter, Capability, CommandType, Domain, ObjectId, ObjectLabel, Session, SessionError, WrapAlg,
};

/// Generate a new wrap key within the `YubiHSM2`
///
/// Delegated capabilities are the set of `Capability` bits that an object is allowed to have
/// when imported or exported using the wrap key
pub fn generate_wrap_key<A: Adapter>(
    session: &mut Session<A>,
    key_id: ObjectId,
    label: ObjectLabel,
    domains: Domain,
    capabilities: Capability,
    delegated_capabilities: Capability,
    algorithm: WrapAlg,
) -> Result<ObjectId, SessionError> {
    session
        .send_command(GenWrapKeyCommand {
            params: GenerateKeyParams {
                key_id,
                label,
                domains,
                capabilities,
                algorithm: algorithm.into(),
            },
            delegated_capabilities,
        }).map(|response| response.key_id)
}

/// Request parameters for `commands::generate_wrap_key`
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GenWrapKeyCommand {
    /// Common parameters to all key generation commands
    pub params: GenerateKeyParams,

    /// Delegated capabilities
    pub delegated_capabilities: Capability,
}

impl Command for GenWrapKeyCommand {
    type ResponseType = GenWrapKeyResponse;
}

/// Response from `commands::generate_wrap_key`
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GenWrapKeyResponse {
    /// ID of the key
    pub key_id: ObjectId,
}

impl Response for GenWrapKeyResponse {
    const COMMAND_TYPE: CommandType = CommandType::GenerateWrapKey;
}
