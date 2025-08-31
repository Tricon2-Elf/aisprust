#pragma once

#include "vce.h"
#include <cstdint>

namespace ivce
{

	class iSocket
	{
  public:
		virtual ~iSocket();
		virtual void F1() = 0;
		virtual void F2() = 0;
		virtual void Delete();

    iSocket* m_pSelf; //0x0004
    uint8_t m_CloseFlags; //0x0008
    char _0x0009[0x3];
    uint32_t _0x000c;
    int m_Socket; //0x0010
    class iVCE* m_pVce; //0x0014
    uint32_t _0x0018;
    uint32_t _0x001c;
    uint32_t _0x0020;
    uint32_t _0x0024;
    uint32_t _0x0028;
    uint32_t _0x002c;
    uint32_t _0x0030;
    uint32_t _0x0034;

	}; //0x0038

	class iSession
    : public iSocket
	{
  public:
		virtual void F4();
		virtual void F5();
		virtual void F6();
		virtual void F7();
		virtual void F8();
		virtual void F9();

		virtual void F10();
		virtual void F11();
		virtual void F12();
		virtual void F13();
		virtual void F14();
		virtual void F15();
		virtual void F16();
		virtual void F17();
		virtual void F18();
		virtual void F19();

		virtual void F20();
		virtual void F21();
		virtual void F22();
		virtual void F23();
		virtual void F24();
		virtual void F25();
		virtual void F26();
		virtual void F27();
		virtual void F28();
		virtual void F29();


    uint32_t _0x0038;
    uint32_t _0x003c;
    uint32_t _0x0040;
    uint32_t _0x0044;
    uint32_t _0x0048;
    uint32_t _0x004c;
    uint32_t _0x0050;
    uint32_t _0x0054;
    uint32_t m_TargetIp; //0x0058
    uint32_t m_SocketIp; //0x005c
    uint32_t _0x0060;
    uint32_t _0x0064;
    uint16_t m_ServerPort; //0x0068
    uint16_t m_SocketPort; //0x006a
    uint32_t _0x006c;
    uint32_t _0x0070;
    uint32_t m_Timeout; //0x0074
    uint32_t _0x0078;
    uint32_t _0x007c;
    uint32_t _0x0080;
    uint32_t _0x0084;
    uint32_t _0x0088;
    char m_ServerIp[256];
    uint32_t m_pCallback; //0x018c
    uint32_t _0x0190;
    uint32_t _0x0194;
    uint32_t _0x0198;
    uint32_t _0x019c;
    uint32_t _0x01a0;
    uint32_t _0x01a4;
    uint32_t _0x01a8;
    uint32_t _0x01ac;

    // TODO: buffer
    
	};

	class iUdpStream
    : public iSession
	{
  public:
	};

	template <class TSession, typename Tcrypt>
	class iCryptSession
    : public TSession
	{
	};

	class iVCE
    : public vce::VCE
	{
  public:
	};




} // namespace ivce
