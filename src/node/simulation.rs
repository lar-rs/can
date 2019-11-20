/// Simulation hardware.
use super::error::CanError;
//pub async fn simulate_api_error( ) -> CanError {
   // CanError::Canbus{ msg:"Simulation error".to_owned()}
//}
// Analog value
// Analog Node
// ID [0x2,0x4]
// Index:
//
// * 6000  ParameterName=Uart 0 ObjectType=0x8 SubNumber=4
//     - 0 ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=3 PDOMapping=0
//     - 1 ParameterName=Uart Daten ObjectType=0x7 DataType=0x0009 AccessType=rw DefaultValue=- PDOMapping=0
//     - 2 ParameterName=RX-Length ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
//     - 3 ParameterName=TX-Length ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=79 PDOMapping=0
// * 6010  ParameterName=Uart 1 ObjectType=0Nx8 SubNumber=4
//     - 0 ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=3 PDOMapping=0
//     - 1 ParameterName=Uart Daten ObjectType=0x7 DataType=0x0009 AccessType=rw DefaultValue=- PDOMapping=0
//     - 2 ParameterName=RX-Length ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=0 PDOMapping=0
//     - 3 ParameterName=TX-Length ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=79 PDOMapping=0
// * 6022  ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=5 PDOMapping=0
//     - 1 ParameterName=ext Temperature 1 ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=774 PDOMapping=0*
//     - 2 ParameterName=ext Temperature 2 ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=774 PDOMapping=0*
//     - 3 ParameterName=ext Temperature 3 ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=774 PDOMapping=0*
// * 6101  ParameterName=AD - Values ObjectType=0x8 SubNumber=5
//     - 0 ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=4 PDOMapping=0
//     - 1 ParameterName=Channel 1 ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=3262 PDOMapping=0
//     - 2 ParameterName=Channel 2 ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=3000 PDOMapping=0
//     - 3 ParameterName=Channel 3 ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=1500 PDOMapping=0
//     - 4 ParameterName=Channel 4 ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=1000 PDOMapping=0
// * 6111  ParameterName=AD - Values ObjectType=0x8 SubNumber=2
//     - 0 ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=1 PDOMapping=0
//     - 1 ParameterName=Channel 1 ObjectType=0x7 DataType=0x0006 AccessType=ro DefaultValue=0 PDOMapping=0
// * 6120  ParameterName=1 x 12Bit DA-Conversion ObjectType=0x8 SubNumber=2
//     - 0 ParameterName=Nr of Subobjects ObjectType=0x7 DataType=0x0002 AccessType=ro DefaultValue=1 PDOMapping=0
//     - 1 ParameterName=Channel 1 ObjectType=0x7 DataType=0x0006 AccessType=rw DefaultValue=0 PDOMapping=0
// DataType
// * 0x0001= 	1 - 1 bit boolean
// * 0x0002= 	1 - 8 bit signed integer
// * 0x0003=     1 - 16 bit signed integer
// * 0x0004=     1 - 32 bit signed integer
// * 0x0005=     1 - 8 bit unsigned integer
// * 0x0006= 	1 - 16 bit unsigned integer
// * 0x0007= 	1 - 32 bit unsigned integer
// * 0x0008=	    1 - single precision floating point
// * 0x0009= 	1 - visible string




// Analog input
// pub async fn analog_input16(num :u8) -> Result<u16 ,CanError>{
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }

// pub async fn digital_input16(num: u8) -> Result<bool,CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }

// pub async fn analog_output16(num :u8) -> Result<u16 ,CanError>{

//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }

// pub async fn digital_output16(num: u8) -> Result<u16,CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }

// pub async fn set_analog_output16(num: u8, val : u16) -> Result<(),CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }

// pub async fn set_digital_output16(num:u8,val : u16) -> Result<(),CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }

// pub async fn analog_uart01_read() -> Result<Vec<u8>,CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }
// pub async fn analog_uart02_read() -> Result<Vec<u8>,CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }

// pub async fn motor_uart01_read() -> Result<Vec<u8>,CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }
// pub async fn motor_uart02_read() -> Result<Vec<u8>,CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }
// pub async fn motor_uart03_read() -> Result<Vec<u8>,CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }
// pub async fn motor_uart04_read() -> Result<Vec<u8>,CanError> {
//     Err(CanError::Canbus{msg:"Simulation error".to_owned()})
// }


//
