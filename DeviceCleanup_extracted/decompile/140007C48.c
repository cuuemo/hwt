/*
 * func-name: sub_140007C48
 * func-address: 0x140007c48
 * callers: 0x140003cec
 * callees: 0x140001854
 */

__int64 __fastcall sub_140007C48(LPCWSTR lpApplicationName, __int64 a2, const WCHAR *a3, WORD a4)
{
  unsigned int v7; // ebx
  BOOL v8; // edi
  struct _PROCESS_INFORMATION ProcessInformation; // [rsp+50h] [rbp-AA8h] BYREF
  struct _STARTUPINFOW StartupInfo; // [rsp+70h] [rbp-A88h] BYREF
  WCHAR CommandLine[1288]; // [rsp+E0h] [rbp-A18h] BYREF

  wsprintfW(CommandLine, L"\"%s\" %s", lpApplicationName, a2);
  v7 = 0;
  StartupInfo.lpDesktop = L"default";
  memset(&ProcessInformation, 0, sizeof(ProcessInformation));
  StartupInfo.cb = 104;
  StartupInfo.lpReserved = 0;
  sub_140001854(&StartupInfo.lpTitle, 0, 0x50u);
  if ( a4 )
  {
    StartupInfo.dwFlags |= 1u;
    StartupInfo.wShowWindow = a4;
  }
  v8 = CreateProcessW(lpApplicationName, CommandLine, 0, 0, 0, 0, 0, a3, &StartupInfo, &ProcessInformation);
  if ( v8 )
  {
    CloseHandle(ProcessInformation.hProcess);
    CloseHandle(ProcessInformation.hThread);
  }
  LOBYTE(v7) = v8;
  return v7;
}
