
pub struct QCircuit {
    pub number_of_qubits: i32,
}

impl QCircuit {
    pub fn new(number_of_qubits: i32) -> QCircuit {
        return QCircuit { number_of_qubits: number_of_qubits, }
    }
}
