/*
 * func-name: sub_140008FC8
 * func-address: 0x140008fc8
 * callers: 0x140003010
 * callees: 0x140001854, 0x14000736c, 0x140008c14
 */

__int64 __fastcall sub_140008FC8(__int64 a1)
{
  struct _PROCESS_INFORMATION ProcessInformation; // [rsp+50h] [rbp-6A8h] BYREF
  struct _STARTUPINFOW StartupInfo; // [rsp+70h] [rbp-688h] BYREF
  WCHAR ApplicationName[264]; // [rsp+E0h] [rbp-618h] BYREF
  WCHAR CommandLine[516]; // [rsp+2F0h] [rbp-408h] BYREF

  wsprintfW(ApplicationName, L"%s\\System32\\rundll32.exe", &Buffer);
  wsprintfW(CommandLine, L"\"%s\" url,OpenURL %s", ApplicationName, a1);
  if ( dword_1400107CC && dword_140010668 >= 6 && (unsigned int)sub_14000736C() )
    return sub_140008C14((__int64)ApplicationName, (__int64)CommandLine, (__int64)&PathName, 3);
  memset(&ProcessInformation, 0, sizeof(ProcessInformation));
  StartupInfo.cb = 104;
  StartupInfo.lpDesktop = L"default";
  StartupInfo.lpReserved = 0;
  sub_140001854(&StartupInfo.lpTitle, 0, 0x50u);
  if ( !CreateProcessW(ApplicationName, CommandLine, 0, 0, 0, 0, 0, 0, &StartupInfo, &ProcessInformation) )
    return 0;
  CloseHandle(ProcessInformation.hProcess);
  CloseHandle(ProcessInformation.hThread);
  return 1;
}
