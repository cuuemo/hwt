/*
 * func-name: start
 * func-address: 0x1400017a8
 * callers: none
 * callees: 0x140001854, 0x1400052dc
 */

void __noreturn start()
{
  const WCHAR *CommandLineW; // rax
  LPWSTR i; // rdi
  unsigned int wShowWindow; // ebx
  HMODULE ModuleHandleW; // rax
  UINT v4; // eax
  _STARTUPINFOW StartupInfo; // [rsp+20h] [rbp-78h] BYREF

  StartupInfo.lpReserved = 0;
  StartupInfo.cb = 104;
  hHeap = GetProcessHeap();
  sub_140001854(&StartupInfo.lpDesktop, 0, 88);
  CommandLineW = GetCommandLineW();
  for ( i = PathGetArgsW(CommandLineW); *i == 32; ++i )
    ;
  GetStartupInfoW(&StartupInfo);
  wShowWindow = 10;
  if ( (StartupInfo.dwFlags & 1) != 0 )
    wShowWindow = StartupInfo.wShowWindow;
  ModuleHandleW = GetModuleHandleW(0);
  v4 = sub_1400052DC(ModuleHandleW, 0, i, wShowWindow, StartupInfo.cb, StartupInfo.lpReserved);
  ExitProcess(v4);
}
