#include "sdk/item_table.h"
#define _WINSOCKAPI_

#include "log.h"

#include <cstdint>
#include <cstdio>
#include <string>
#include <intrin.h>

#include <detours.h>

#include "sdk/aistd.h"
#include "sdk/ivce.h"
#include "sdk/vce.h"

#include <Windows.h>

inline LONG AttachDetour(void* targetAddr, void** targetOrig, void* destFunc)
{
	*targetOrig = targetAddr;
	return DetourAttach(targetOrig, destFunc);
}


namespace HookCreateStream
{

	using tCreateStream = ivce::iSession*(__cdecl*)(ivce::iVCE * pVce,
	                                                vce::Session* pSession,
	                                                uintptr_t a3,
	                                                uintptr_t a4);

	tCreateStream oCreateStream = nullptr;

	ivce::iSession* __cdecl hkCreateStream(ivce::iVCE* pVce,
	                                       vce::Session* pSession,
	                                       uintptr_t a3,
	                                       uintptr_t a4)
	{
		// InitConsole();
		printf("[%s] %p %p %p %p\n", __FUNCTION__, pVce, pSession, a3, a4);
		printf("\tNetAlgo[%i]\n", pSession->m_NetAlgo);
		printf("\tStreamType[%i]\n", pSession->m_StreamType);

    // pSession->m_StreamType = vce::ST_UDP;
    pSession->m_NetAlgo = 0x04; //normal tcp

		return oCreateStream(pVce, pSession, a3, a4);
	}
} // namespace HookCreateStream


namespace HookConnectWsaError
{
  
	using tGetConnectError = int (__cdecl*)();

	tGetConnectError oGetConnectError = nullptr;

	int __cdecl hkGetConnectError()
	{


    // for some reason wine doesnt like this. so we just always return no error.. breaks some ohther thigngs toh
		int codenz =  oGetConnectError();

    printf("[%s] %i %i\n", __FUNCTION__, codenz, WSAGetLastError());
  
    return 0;
	}
}

namespace HookGetString
{

	using tGetString = aistd::wstring*(__cdecl*)(uint32_t string_id);

	tGetString oGetString = nullptr;

	aistd::wstring* __cdecl hkGetString(uint32_t string_id)
	{
		aistd::wstring* pString = oGetString(string_id);

		// InitConsole();
		printf("[0x%x][0x%x] %ls\n", string_id, _ReturnAddress(), pString->c_str());

		// ip and port is randomy chosen
		// 0x640FAA1B = 119.75.227.141 = auth ip 1 
		// 0x640FAA1d = 119.75.227.142 = auth ip 2

		return pString;
	}
} // namespace HookGetString



namespace HookLog
{

	using tLogHandler = void(__cdecl*)(vce::LogType logType, vce::LogEntry* pEntry);

	tLogHandler oLogHandler = nullptr;

	void __cdecl hkLogHandler(vce::LogType logType, vce::LogEntry* pEntry)
	{
    aistd::string LogMessage;
    vce::log(LogMessage, logType, pEntry);

    if (logType != vce::VCE_LOG_TYPE_SOCKET_POLL_CHECK)
      printf("%s\n", LogMessage.c_str());

		return oLogHandler(logType, pEntry);
	}
} // namespace HookCreateStream


namespace HookItemList
{

	using tInitList = void(__thiscall*)(CItemTable* pThis, ITEM_DATA* pData, uint32_t count);
	tInitList oInitList = nullptr;

	void __fastcall hkInitList(CItemTable* pThis, void*, ITEM_DATA* pData, uint32_t count)
	{
    for(auto i=0; i < count; i++)
    {
      printf("[%s] %i %i %s\n", __FUNCTION__, pData[i].key, pData[i].item_id, pData[i].name);
    }
    printf("[%s] count = %i %s\n", __FUNCTION__, count);
    oInitList(pThis, pData, count);
	}
} // namespace HookItemList


namespace HookAuthPacket
{

	using tAuthRecvPkt = bool(__thiscall*)(void* pThis, uint8_t* pData, uint32_t size);

	tAuthRecvPkt oAuthRecvPkt = nullptr;

	bool __fastcall hkAuthRecvPkt(void* pThis,void*, uint8_t* pData, uint32_t size)
	{
    printf("\n[CAIProtoAuth::RecvPkt][0x%p] MsgId=0x%04x size=0x%x\n", _ReturnAddress(), *reinterpret_cast<uint16_t*>(pData), size );

    return oAuthRecvPkt(pThis, pData, size);
	}
} // namespace HookCreateStream


BOOL APIENTRY DllMain(HMODULE hModule, DWORD dwReason, LPVOID lpReserved)
{

	switch (dwReason)
	{
	case DLL_PROCESS_ATTACH:
	{

		DisableThreadLibraryCalls(hModule);

		DetourTransactionBegin();
		DetourUpdateThread(GetCurrentThread());

		uintptr_t Module =
		    reinterpret_cast<uintptr_t>(GetModuleHandleA(nullptr));

		AttachDetour(reinterpret_cast<void*>(Module + (0x8AD3E0 - 0x400000)),
		             reinterpret_cast<void**>(&HookCreateStream::oCreateStream),
		             &HookCreateStream::hkCreateStream);

		AttachDetour(reinterpret_cast<void*>(Module + (0x8680D0 - 0x400000)),
		             reinterpret_cast<void**>(&HookGetString::oGetString),
		             &HookGetString::hkGetString);

		AttachDetour(reinterpret_cast<void*>(Module + (0x897040 - 0x400000)),
		             reinterpret_cast<void**>(&HookLog::oLogHandler),
		             &HookLog::hkLogHandler);


		// AttachDetour(reinterpret_cast<void*>(Module + (0x7F6DC0 - 0x400000)),
		//              reinterpret_cast<void**>(&HookAuthPacket::oAuthRecvPkt),
		//              &HookAuthPacket::hkAuthRecvPkt);
		//

		AttachDetour(reinterpret_cast<void*>(Module + (0x47CD20 - 0x400000)),
		             reinterpret_cast<void**>(&HookItemList::oInitList),
		             &HookItemList::hkInitList);

		AttachDetour(reinterpret_cast<void*>(Module + (0x8B4930 - 0x400000)),
		             reinterpret_cast<void**>(&HookConnectWsaError::oGetConnectError),
		             &HookConnectWsaError::hkGetConnectError);

		DetourTransactionCommit();

		break;
	}
	case DLL_THREAD_ATTACH:
		// Code to run when a new thread is created
		break;
	case DLL_THREAD_DETACH:
		// Code to run when a thread exits cleanly
		break;
	case DLL_PROCESS_DETACH:
	{
		DetourTransactionBegin();
		DetourUpdateThread(GetCurrentThread());

		// this will hook the function
		DetourDetach(reinterpret_cast<void**>(&HookCreateStream::oCreateStream),
		             &HookCreateStream::hkCreateStream);

		DetourTransactionCommit();
		// Code to run when the DLL is unloaded
		break;
	}
	}
	return TRUE;
}
