

typedef unsigned int U32;
typedef unsigned char U8;
typedef int Err;
typedef unsigned char TYPE;
typedef struct Data{
    U8 buf[256];
    unsigned char len;
}Data;

typedef struct Frame{
    TYPE type;
    char buf[256];
    U8 len;
}Frame;

Err     close_can0();
U32     read_unsigned  (int node,int index,U8 sub);
Err     write_unsigned (int node,int index,U8 sub, TYPE type, U32 value);
Err     canframe         (int node,int index,U8 sub, Frame *frame);
/// Analog node interface
U32     analog_get_in01  (int node);
U32     analog_get_in02  (int node);
U32     analog_get_in03  (int node);
U32     analog_get_in04  (int node);
U32     analog_get_in05  (int node);
U32     analog_get_out   (int node);
Err     analog_set_out   (int node,U32 value);
U32     analog_get_temp01(int node);
U32     analog_get_temp02(int node);
U32     analog_get_temp03(int node);
Data    analog_get_uart01(int node);
Err     analog_set_baut01(int node,U32 baut);
Err     analog_set_uart01(int node,U8 *data);
Data    analog_get_uart02(int node);
Err     analog_set_baut02(int node,U32 baut);
Err     analog_set_uart02(int node,U8 *data);

/// Digital node
/// Nodes: [Digital1:0x18,Digital2:0x19,Digital3:0x1a]
/// Input 16bit min 0x0000 max 0xffff:0b1111_1111_1111_1111
U32     digital_get_input(int node);
/// Output 16bib
U32     digital_get_output(int node);
/// Change output 16bit
Err     digital_set_output(int node,U32 value);

/// Doppelmotor node interfaces.
/// Nodes ID [Doppelmotor1:0x12,Doppelmotor2:0x14]
///
/// Read UartData01
/// Index: 0x6000,0x1
const char* doppelmotor_get_uart01(int node);
/// Read UartData02
/// Address: 0x6010,0x1
const char* doppelmotor_get_uart02(int node);
/// Change bautrate for UartData01
/// Address: 0x6000,0x4
/// Value:
Err     doppelmotor_set_baut01(int node,unsigned baut);
/// Change bautrate for UartData02
/// Address: 0x6010,0x4
/// Value:  9 = 9600
///           
Err     doppelmotor_set_baut02(int node,unsigned baut);
/// Write string
Err     doppelmotor_set_uart01(int node,const char *data);
/// Write string
Err     doppelmotor_set_uart02(int node,const char *data);

/// Analog extension modul
/// NodeID: Analogext:0x1C
///
/// count real analog outputs bounds on extension modul
U8      analogext_get_count();

/// Read analog out value u16 from modul number
/// out range [1..max]
/// max - read with `analogext_get_count` function.
U32     analogext_get_out  (U8 out);

/// Change real analog output value on modul.
/// out range [1..max]
/// max - read with `analogext_get_count` function.
Err     analogext_set_out  (U8 out ,U32 value);
