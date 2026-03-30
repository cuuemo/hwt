/*
 * func-name: sub_140006AC0
 * func-address: 0x140006ac0
 * callers: 0x140002a74, 0x140006378
 * callees: 0x140001854
 */

__int64 sub_140006AC0()
{
  HMODULE ModuleHandleA; // rax
  FARPROC RtlGetVersion; // rbx
  int v3; // [rsp+20h] [rbp-128h] BYREF
  unsigned int v4; // [rsp+24h] [rbp-124h] BYREF
  unsigned int v5; // [rsp+28h] [rbp-120h]

  ModuleHandleA = GetModuleHandleA("ntdll.dll");
  RtlGetVersion = GetProcAddress(ModuleHandleA, "RtlGetVersion");
  if ( RtlGetVersion )
  {
    v3 = 284;
    sub_140001854(&v4, 0, 0x118u);
    if ( !((unsigned int (__fastcall *)(int *))RtlGetVersion)(&v3) )
      return v5 + ((v4 + 6 * (v4 / 0xA)) << 8) + 6 * (v5 / 0xA);
  }
  __debugbreak();
  return 4096;
}
