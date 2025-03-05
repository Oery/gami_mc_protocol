use gami_macros::packet;

#[packet(0x3D, server)]
pub struct ScoreboardDisplay {
    pub position: ScoreboardPosition,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[encoding("u8")]
pub enum ScoreboardPosition {
    List = 0,
    Sidebar = 1,
    BelowName = 2,
    SidebarTeamBlack = 3,
    SidebarTeamDarkBlue = 4,
    SidebarTeamDarkGreen = 5,
    SidebarTeamDarkAqua = 6,
    SidebarTeamDarkRed = 7,
    SidebarTeamDarkPurple = 8,
    SidebarTeamGold = 9,
    SidebarTeamGray = 10,
    SidebarTeamDarkGray = 11,
    SidebarTeamBlue = 12,
    SidebarTeamGreen = 13,
    SidebarTeamAqua = 14,
    SidebarTeamRed = 15,
    SidebarTeamLightPurple = 16,
    SidebarTeamYellow = 17,
    SidebarTeamWhite = 18,
}
