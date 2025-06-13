/// BGPのRFC内 8.1
/// (https://datatracker.ietf.org/doc/html/rfc4271#section-8.1)で
/// 定義されているEventを表す列挙型です。
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
    ManualStart,
    // 正常系しか実装しない本実装では別のEventとして扱う意味がないため、
    // TcpConnectionConfirmedはTcpCrAckedも兼ねている。
    TcpConnectionConfirmed,
}
