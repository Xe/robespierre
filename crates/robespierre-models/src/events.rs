use serde::{Deserialize, Serialize};

use crate::{
    channel::{Channel, ChannelField, Message, PartialChannel, PartialMessage},
    id::{ChannelId, MemberId, MessageId, RoleId, ServerId, UserId},
    server::{
        Member, MemberField, PartialMember, PartialRole, PartialServer, RoleField, Server,
        ServerField,
    },
    user::{PartialUser, RelationshipStatus, User, UserField},
};

/// Any message the client can send to the server.
#[derive(Serialize, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[serde(tag = "type")]
pub enum ClientToServerEvent {
    Authenticate {
        user_id: UserId,
        session_token: String,
    },

    #[serde(rename = "Authenticate")]
    AuthenticateBot {
        token: String,
    },

    BeginTyping {
        channel: ChannelId,
    },
    EndTyping {
        channel: ChannelId,
    },
    Ping {
        time: u32,
        #[deprecated(note = "Temporary fix for temporary issue")]
        data: (u8,),
    },
}

/// Event received after authentication.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ReadyEvent {
    pub users: Vec<User>,
    pub servers: Vec<Server>,
    pub channels: Vec<Channel>,
    pub members: Vec<Member>,
}

/// Any message that the server can send to the client.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum ServerToClientEvent {
    Error {
        error: String,
    },
    Authenticated,
    Pong {
        time: u32,
    },
    Ready {
        #[serde(flatten)]
        event: ReadyEvent,
    },
    Message {
        #[serde(flatten)]
        message: Message,
    },
    MessageUpdate {
        id: MessageId,
        channel: ChannelId,
        data: PartialMessage,
    },
    MessageDelete {
        id: MessageId,
        channel: ChannelId,
    },
    ChannelCreate {
        #[serde(flatten)]
        channel: Channel,
    },
    ChannelUpdate {
        id: ChannelId,
        data: PartialChannel,
        #[serde(default)]
        clear: Option<ChannelField>,
    },
    ChannelDelete {
        id: ChannelId,
    },
    ChannelGroupJoin {
        id: ChannelId,
        user: UserId,
    },
    ChannelGroupLeave {
        id: ChannelId,
        user: UserId,
    },
    ChannelStartTyping {
        id: ChannelId,
        user: UserId,
    },
    ChannelStopTyping {
        id: ChannelId,
        user: UserId,
    },
    ChannelAck {
        id: ChannelId,
        user: UserId,
        message_id: MessageId,
    },
    ServerUpdate {
        id: ServerId,
        data: PartialServer,
        #[serde(default)]
        clear: Option<ServerField>,
    },
    ServerDelete {
        id: ServerId,
    },
    ServerMemberUpdate {
        id: MemberId,
        data: PartialMember,
        #[serde(default)]
        clear: Option<MemberField>,
    },
    ServerMemberJoin {
        id: ServerId,
        user: UserId,
    },
    ServerMemberLeave {
        id: ServerId,
        user: UserId,
    },
    ServerRoleUpdate {
        id: ServerId,
        role_id: RoleId,
        data: PartialRole,
        #[serde(default)]
        clear: Option<RoleField>,
    },
    ServerRoleDelete {
        id: ServerId,
        role_id: RoleId,
    },
    UserUpdate {
        id: UserId,
        data: PartialUser,
        #[serde(default)]
        clear: Option<UserField>,
    },
    UserRelationship {
        id: UserId,
        user: UserId,
        status: RelationshipStatus,
    },
}
