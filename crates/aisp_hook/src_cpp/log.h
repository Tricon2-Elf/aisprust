#pragma once

#define _WINSOCKAPI_

#include <cstdarg>
#include <cstdio>
#include <cstring>
#include <winsock2.h>

#include <ws2tcpip.h>

#define SERVER_IP "127.0.0.1"
#define SERVER_PORT 9999

void udp_printf(const char* fmt, ...)
{
	static SOCKET Sock = INVALID_SOCKET;
	static sockaddr_in ServerAddr = {0};
	static bool Initialized = false;

	if (!Initialized)
	{
		WSADATA WsaData;
		if (WSAStartup(MAKEWORD(2, 2), &WsaData) != 0)
		{
			fprintf(stderr, "WSAStartup failed\n");
			return;
		}

		Sock = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
		if (Sock == INVALID_SOCKET)
		{
			fprintf(stderr, "Socket creation failed: %d\n", WSAGetLastError());
			WSACleanup();
			return;
		}

		ServerAddr.sin_family = AF_INET;
		ServerAddr.sin_port = htons(SERVER_PORT);
		inet_pton(AF_INET, SERVER_IP, &ServerAddr.sin_addr);

		atexit([]()
    {
      closesocket(Sock);
      WSACleanup();
    });


		Initialized = 1;
	}



	char Buffer[1024] = {0};

	va_list Args;
	va_start(Args, fmt);
	auto BufLen = vsnprintf(Buffer, sizeof(Buffer), fmt, Args);
	va_end(Args);

	sendto(Sock, Buffer, BufLen, 0, reinterpret_cast<sockaddr*>(&ServerAddr),
	       sizeof(ServerAddr));
}

#define printf udp_printf
