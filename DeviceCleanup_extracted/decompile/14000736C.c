/*
 * func-name: sub_14000736C
 * func-address: 0x14000736c
 * callers: 0x140008fc8
 * callees: none
 */

__int64 sub_14000736C()
{
  HMODULE ModuleHandleW; // rax
  FARPROC RtlQueryElevationFlags; // rax
  int v3; // [rsp+30h] [rbp+8h] BYREF

  if ( (dword_140016FD8 & 1) != 0 )
  {
    RtlQueryElevationFlags = (FARPROC)qword_140016FD0;
  }
  else
  {
    dword_140016FD8 |= 1u;
    ModuleHandleW = GetModuleHandleW(L"ntdll");
    RtlQueryElevationFlags = GetProcAddress(ModuleHandleW, "RtlQueryElevationFlags");
    qword_140016FD0 = (__int64)RtlQueryElevationFlags;
  }
  if ( !RtlQueryElevationFlags )
    return 0;
  v3 = 0;
  if ( ((unsigned int (__fastcall *)(int *))RtlQueryElevationFlags)(&v3) )
    return 0;
  else
    return v3 & 1;
}
