/**
 * bind connect
 *  0       0   -> Closed
 *  0       1   -> Bound
 *  1       0   -> Available
 *  1       1   -> Uknown
 */
pub enum PortState {
    Closed,
    Bound,
    Available,
    Uknown
}

impl PortState {

    pub fn match_state(bind: bool, conn: bool) -> PortState {
        let _m = match (bind, conn) {
            (false, false) => PortState::Closed,
            (false, true) => PortState::Bound,
            (true, false) => PortState::Available,
            (true, true) => PortState::Uknown,
        };
        return _m;
    }
    
    pub fn is_available(&self) -> bool {
        match self {
            PortState::Available => true,
            _ => false,
        }
    }
    pub fn is_bound(&self) -> bool {
        match self {
            PortState::Bound => true,
            _ => false,
        }
    }

    pub fn is_closed(&self) -> bool {
        match self {
            PortState::Closed => true,
            _ => false,
        }
    }
}