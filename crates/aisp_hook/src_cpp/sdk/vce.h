#pragma once

#include <cstdint>
#include <stdint.h>
#include "aistd.h"

namespace vce
{
  enum StreamType : int
  {
    ST_TCP,
    ST_UDP,
    ST_LOOPBACK,
  };

  enum EncryptionType
  {
    ET_CAMELLIA128=1,
    ET_BLOWFISH128,
    ET_RIJNDAEL128,
    ET_NONE,
  };

  enum CompressionType 
  {
    CT_NONE,
    CT_LZP,
    CT_ZLIB,
  };

	class Base
	{
  public:
		virtual ~Base();

		virtual bool Attached() { return false; }
		virtual bool Detached() { return false; }
		virtual void Think() {}
		virtual int32_t GetState()
		{
			return 0;
		}
		virtual void Exception() {}

		uint32_t _0x0004;
		int32_t m_Id;           // 0x0008
		void* m_pAllocator; // 0x000c
	}; //0x0010

	class Session
    : public Base
	{
  public:
		~Session() {}
    
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
    
    int m_NetAlgo; //0x0010
    StreamType m_StreamType; //0x0014
    int _0x0018; //0x0018
    int _0x001c; //0x001c
    int _0x0020; //0x0020
    int _0x0024; //0x0024
	}; //0x0028

  class Codec
    : public Session
  {
  public:
    virtual void F20();
    virtual void F21();
    virtual void F22();
    virtual void F23();

    
    int _0x0028; //0x0028
    int _0x002c; //0x002c

    int _0x0030; //0x0030
    int _0x0034; //0x0034
    uint64_t _0x0038; //0x0038

    int _0x0040; //0x0040
    int _0x0044; //0x0044
    uint64_t _0x0048; //0x0048
  
    int _0x0050; //0x0050
  };

	class VCE
	{
  public:
		virtual ~VCE();
		virtual void Poll();
		virtual void F2(int32_t);
		virtual void F3(uint32_t);
		virtual void SetTcpCreateFunc(void* pFunc);

    // type 0
		virtual int32_t CreateConnection1(Base* pBase,
		                                 wchar_t* pServerIp,
		                                 uint16_t serverPort,
		                                 int timeout,
		                                 int a5);
		virtual int32_t CreateConnection2(Base* pBase,
		                                 wchar_t pServerIp[256],
		                                 uint16_t serverPort,
		                                 int timeout,
		                                 int a5);


    // type 1
		virtual int32_t CreateConnection3(Base* pBase,
		                                 wchar_t* pServerIp,
		                                 uint16_t serverPort,
		                                 int timeout,
		                                 int a5);
		virtual int32_t CreateConnection4(Base* pBase,
		                                 wchar_t pServerIp[256],
		                                 uint16_t serverPort,
		                                 int timeout,
		                                 int a5);

		virtual int32_t CreateConnectionRaw(Base* pBase, wchar_t pServerIp[256], uint16_t port);

		virtual void CreateStreamSession(void*, int, int);
		virtual void CreateTcpListenerW(void* , uint16_t listenPort, wchar_t* pListenIp);
		virtual void CreateTcpListenerA(void*, uint16_t listenPort, char* pListenIp);
		virtual void CreateTcpListener(void*, uint16_t listenPort);
		virtual void CreateUdpListenerW(void*, uint16_t listenPort, wchar_t* pListenIp);
		virtual void CreateUdpListenerA(void*, uint16_t listenPort, char* pListenIp);
		virtual void CreateUdpListener(void*, uint16_t listenPort);

		virtual void CreateProxyListener(void*, uint16_t port, char* listenIp, char* a5 );
		virtual void CreateLoopbackListener(void*, uint32_t);
		virtual void CreateLocalFinder(void*, uint16_t, char* name, uint16_t);

		virtual uint32_t F20(uint32_t socketId);
		virtual uint32_t F21(uint32_t socketId);
		virtual void F22(void* pVector);

		virtual void F23(bool );
		virtual void F24(int);
		virtual void F25(int);
		virtual uint32_t F26(); // count of some array
		virtual bool HasSession(int sessionId);
		virtual void F28();
		virtual void PollStreams(uint64_t time);

		virtual void F30(int);
		virtual void F31();
		virtual void F32();
		virtual void F33(int);
		virtual void F34(int);
		virtual void F35(uint64_t);
		virtual void F36(bool);
	};


enum LogType
{
  VCE_LOG_TYPE_SOCKET_REUSE_ADDRESS = 0x1,
  VCE_LOG_TYPE_SOCKET_SET_NONBLOCKING = 0x2,
  VCE_LOG_TYPE_SOCKET_ACCEPT_TCP_SOCKET = 0x3,
  VCE_LOG_TYPE_SOCKET_BIND_SOCKET = 0x4,
  VCE_LOG_TYPE_SOCKET_CLOSE_SOCKET = 0x5,
  VCE_LOG_TYPE_SOCKET_CONNECT_SOCKET = 0x6,
  VCE_LOG_TYPE_SOCKET_CREATE_TCP_SOCKET = 0x7,
  VCE_LOG_TYPE_SOCKET_GET_LOCAL_ADDRESS = 0x8,
  VCE_LOG_TYPE_SOCKET_LISTEN_SOCKET = 0x9,
  VCE_LOG_TYPE_SOCKET_RECV_TCP_SOCKET = 0xA,
  VCE_LOG_TYPE_SOCKET_SEND_TCP_SOCKET = 0xB,
  VCE_LOG_TYPE_SOCKET_SET_NODELAY = 0xC,
  VCE_LOG_TYPE_SOCKET_SHUTDOWN_RECV_SOCKET = 0xD,
  VCE_LOG_TYPE_SOCKET_SHUTDOWN_SEND_SOCKET = 0xE,
  VCE_LOG_TYPE_SOCKET_SHUTDOWN_SOCKET = 0xF,
  VCE_LOG_TYPE_SOCKET_CREATE_UDP_SOCKET = 0x10,
  VCE_LOG_TYPE_SOCKET_GET_SEND_QUEUE_SIZE = 0x11,
  VCE_LOG_TYPE_SOCKET_SEND_TO = 0x12,
  VCE_LOG_TYPE_SOCKET_RECV_FROM = 0x13,
  VCE_LOG_TYPE_SOCKET_SET_MULTICAST_INTERFACE = 0x14,
  VCE_LOG_TYPE_SOCKET_POLL_CHECK = 0x15,
  VCE_LOG_TYPE_CODEC_SENDED_PING = 0x16,
  VCE_LOG_TYPE_CODEC_RECEIVED_PING_AND_SENDED_PONG = 0x17,
  VCE_LOG_TYPE_CODEC_RECEIVED_PONG = 0x18,
  VCE_LOG_TYPE_CODEC_TERMINATED = 0x19,
  VCE_LOG_TYPE_CODEC_TIMEOUT = 0x1A,
  VCE_LOG_TYPE_CODEC_SENDED_PUBLIC_KEY = 0x1B,
  VCE_LOG_TYPE_CODEC_GENERATED_PRIVATE_KEY = 0x1C,
  VCE_LOG_TYPE_CODEC_GENERATED_PUBLIC_KEY = 0x1D,
  VCE_LOG_TYPE_CODEC_TIMEOUT_KEY_EXCHANGE = 0x1E,
  VCE_LOG_TYPE_CODEC_RECEIVED_PUBLIC_KEY = 0x1F,
  VCE_LOG_TYPE_CODEC_GENERATED_CRYPTIC_SHARED_KEY = 0x20,
  VCE_LOG_TYPE_CODEC_RECEIVED_CRYPTIC_SHARED_KEY = 0x21,
  VCE_LOG_TYPE_CODEC_SENDED_ENCRYPTION_KEY = 0x22,
  VCE_LOG_TYPE_CODEC_SENDED_DECRYPTION_KEY = 0x23,
  VCE_LOG_TYPE_CODEC_RECEIVED_ENCRYPTION_KEY = 0x24,
  VCE_LOG_TYPE_CODEC_RECEIVED_DECRYPTION_KEY = 0x25,
  VCE_LOG_TYPE_CODEC_WAITING_KEY_EXCHANGE = 0x26,
  VCE_LOG_TYPE_CODEC_INVALID_CODEC = 0x27,
  VCE_LOG_TYPE_LISTENER_CREATED = 0x28,
  VCE_LOG_TYPE_LISTENER_ACCEPTED_SESSION = 0x29,
  VCE_LOG_TYPE_LISTENER_UDP_RECEIVED_SYN = 0x2A,
  VCE_LOG_TYPE_LISTENER_UDP_SENDED_SYNACK = 0x2B,
  VCE_LOG_TYPE_LISTENER_UDP_RECEIVED_SYNACKACK = 0x2C,
  VCE_LOG_TYPE_LISTENER_SENDED_RECONNECTION_REQUEST = 0x2D,
  VCE_LOG_TYPE_LISTENER_OVER_BACKLOG = 0x2E,
  VCE_LOG_TYPE_LISTENER_DESTROYED = 0x2F,
  VCE_LOG_TYPE_SESSION_CREATED = 0x30,
  VCE_LOG_TYPE_SESSION_RECV_BUFFER_EMPTY = 0x31,
  VCE_LOG_TYPE_SESSION_FORCE_CLOSE = 0x32,
  VCE_LOG_TYPE_SESSION_CLOSE = 0x33,
  VCE_LOG_TYPE_SESSION_WAITING_TO_CLOSE_BECAUSE_BUFFER_EMPTY = 0x34,
  VCE_LOG_TYPE_SESSION_TRYING_NAME_RESOLUTION = 0x35,
  VCE_LOG_TYPE_SESSION_COMPLETE_NAME_RESOLUTION = 0x36,
  VCE_LOG_TYPE_SESSION_CONNECTION_CANCELED = 0x37,
  VCE_LOG_TYPE_SESSION_CONNECTION_TIMEOUT = 0x38,
  VCE_LOG_TYPE_SESSION_CONNECTION_RETRIED = 0x39,
  VCE_LOG_TYPE_SESSION_CONNECTION_RESTARTED = 0x3A,
  VCE_LOG_TYPE_SESSION_CHECKING_CONNECTION_REFUSING = 0x3B,
  VCE_LOG_TYPE_SESSION_CONNECTION_REFUSED = 0x3C,
  VCE_LOG_TYPE_SESSION_CONNECTSOCKET_IGNORED_ERROR = 0x3D,
  VCE_LOG_TYPE_SESSION_RECV_FUNC_FAILED = 0x3E,
  VCE_LOG_TYPE_SESSION_CONNECTION_COMPLETE = 0x3F,
  VCE_LOG_TYPE_SESSION_UDP_SENDED_SYN = 0x40,
  VCE_LOG_TYPE_SESSION_UDP_RECEIVED_SYNACK = 0x41,
  VCE_LOG_TYPE_SESSION_UDP_SENDED_SYNACKACK = 0x42,
  VCE_LOG_TYPE_SESSION_UDP_RECEIVED_RECONNECTION_REQUEST = 0x43,
  VCE_LOG_TYPE_SESSION_UDP_SENDED_HANDSHAKING_PING = 0x44,
  VCE_LOG_TYPE_SESSION_UDP_COMPLETE_HANDSHAKING = 0x45,
  VCE_LOG_TYPE_SESSION_UDP_RECEIVED_HANDSHAKING_PING_AND_SENDED_PONG = 0x46,
  VCE_LOG_TYPE_SESSION_UDP_TIMEOUT_RECEIVING = 0x47,
  VCE_LOG_TYPE_SESSION_UDP_CONNECTION_COMPLETE = 0x48,
  VCE_LOG_TYPE_SESSION_UDP_CONNECTED_SEQUENCE = 0x49,
  VCE_LOG_TYPE_SESSION_UDP_SENDED_CLOSE_COMMAND = 0x4A,
  VCE_LOG_TYPE_SESSION_UDP_RECEIVED_CLOSE_COMMAND = 0x4B,
  VCE_LOG_TYPE_SESSION_UDP_SENDED_PING = 0x4C,
  VCE_LOG_TYPE_SESSION_UDP_RECEIVED_PING = 0x4D,
  VCE_LOG_TYPE_SESSION_UDP_SENDED_DATA = 0x4E,
  VCE_LOG_TYPE_SESSION_UDP_RECEIVED_DATA = 0x4F,
  VCE_LOG_TYPE_SESSION_UDP_SENDED_UNRELIABLE_DATA = 0x50,
  VCE_LOG_TYPE_SESSION_UDP_RECEIVED_UNRELIABLE_DATA = 0x51,
  VCE_LOG_TYPE_SESSION_UDP_SENDED_FLAGMENT_COMMAND = 0x52,
  VCE_LOG_TYPE_SESSION_UDP_RECEIVED_FLAGMENT_COMMAND = 0x53,
  VCE_LOG_TYPE_SESSION_UDP_SENDED_ACK = 0x54,
  VCE_LOG_TYPE_SESSION_DESTROYED = 0x55,
  VCE_LOG_TYPE_VCE_CREATED = 0x56,
  UNK_87 = 0x57,
  VCE_LOG_TYPE_VCE_OS_IS_USING_PORT = 0x58,
  VCE_LOG_TYPE_VCE_SELECT_ALGORITHM_CALLBACK_IS_NULL = 0x59,
  VCE_LOG_TYPE_VCE_POLLING_TIME_IS_TOO_LONG = 0x5A,
  VCE_LOG_TYPE_TIME_OF_CLOSING_SOCKET_IS_TOO_LONG = 0x5B,
  VCE_LOG_TYPE_START_DNS_THREAD = 0x5C,
  VCE_LOG_TYPE_END_DNS_THREAD = 0x5D,
  VCE_LOG_TYPE_PROXY_LISTENER_CATCHED_EXCEPTION_ON_CONNECTION_TO_PROXYTOOL = 0x5E,
  VCE_LOG_TYPE_PROXY_LISTENER_CONNECTED_TO_PROXYTOOL = 0x5F,
  VCE_LOG_TYPE_PROXY_LISTENER_CLOSED_CONNECTION_PROXYTOOL = 0x60,
  VCE_LOG_TYPE_UPNP_PREP = 0x61,
  VCE_LOG_TYPE_UPNP_CHECK_ROUTER = 0x62,
  VCE_LOG_TYPE_UPNP_CONNECTING_ROUTER_PREP = 0x63,
  VCE_LOG_TYPE_UPNP_CONNECTING_TO_ROUTER = 0x64,
  VCE_LOG_TYPE_UPNP_GET_ROUTER_SPEC = 0x65,
  VCE_LOG_TYPE_UPNP_DISCONNECT_FROM_ROUTER = 0x66,
  VCE_LOG_TYPE_UPNP_COMPLETE_SPEC = 0x67,
  VCE_LOG_TYPE_UPNP_ACTION_CONNECTING = 0x68,
  VCE_LOG_TYPE_UPNP_ACTION_WAIT_RESPONSE = 0x69,
  VCE_LOG_TYPE_UPNP_ACTION_COMPLETE = 0x6A,
  VCE_LOG_TYPE_UPNP_FAIL = 0x6B,
  VCE_LOG_TYPE_UPNP_UNKNOWN_SEQUENCE = 0x6C,
  VCE_LOG_TYPE_UPNP_EXCEPTION = 0x6D,
  VCE_LOG_TYPE_UPNP_FAILED_RESPONSE = 0x6E,
  VCE_LOG_TYPE_UPNP_MAYBE_NOT_SUPPORT_PORTMAPPING = 0x6F,
  VCE_LOG_TYPE_UPNP_MAYBE_NOT_SUPPORT_QOS = 0x70,
  VCE_LOG_TYPE_HTTP_SESSION_FAILED_TO_OPEN_FILE = 0x71,
  VCE_LOG_TYPE_HTTP_SESSION_SUCCEEDED_TO_OPEN_FILE = 0x72,
  VCE_LOG_TYPE_HTTP_LISTENER_ADDED_AUTHOR = 0x73,
  VCE_LOG_TYPE_STREAM_MULTIPLEXER_PORT_IS_ALREADY_LISTENED = 0x74,
  VCE_LOG_TYPE_STREAM_MULTIPLEXER_PORT_IS_NOT_LISTENED = 0x75,
  VCE_LOG_TYPE_STREAM_MULTIPLEXER_WILD_IS_ALREADY_LISTENED = 0x76,
  VCE_LOG_TYPE_STREAM_MULTIPLEXER_MAINSTREAM_IS_NOT_BOUND = 0x77,
  VCE_LOG_TYPE_STREAM_MULTIPLEXER_MESSAGE_CANNOT_FORMATTED = 0x78,
  VCE_LOG_TYPE_STREAM_MULTIPLEXER_MUXFLOWCONTROL_CANNOT_FORMATTED = 0x79,
  VCE_LOG_TYPE_STREAM_MULTIPLEXER_PURGE_CLOSED_INTERNAL_ERROR = 0x7A,
  VCE_LOG_TYPE_STREAM_MULTIPLEXER_HAS_NO_LISTENER = 0x7B,
  VCE_LOG_TYPE_STREAM_CONNECTION_TO_PORT_IS_REJECTED = 0x7C,
  VCE_LOG_TYPE_STREAM_UNEXPECTED_MESSAGE_TO_MISSING_PORT = 0x7D,
  VCE_LOG_TYPE_STREAM_RECEIVED_MESSAGE_TO_CLOSE_PORT = 0x7E,
  VCE_LOG_TYPE_STREAM_MESSAGE_CONTENT_IS_BROKEN = 0x7F,
};

  class LogEntry
  {
  public:
    class VCE* m_pVce; //0x0000
    uint32_t _0x0004;
    uint32_t _0x0008;
    uint32_t m_Params[];
  };

  aistd::string* log(aistd::string& out, LogType logType, LogEntry* pLogEntry)
  {
    using tLog = aistd::string* (__cdecl*)(aistd::string&, LogType logType, LogEntry* pEntry);
    static tLog fLog = nullptr;

    if (!fLog)
      fLog = reinterpret_cast<tLog>(reinterpret_cast<uintptr_t>(GetModuleHandleA(nullptr)) +(0x894E10 - 0x400000));
    
    return fLog(out, logType, pLogEntry);
  }


  // static uint32_t GetStringId()

} // namespace vce
