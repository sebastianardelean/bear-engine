use crate::circuit::QCircuit;

pub struct EditorState {
    pub qcircuit_parms: QCircuit,
}

impl EditorState {
    pub fn new() -> EditorState {
        return Self {
            qcircuit_parms: QCircuit::new(0),
        };
    }
}
