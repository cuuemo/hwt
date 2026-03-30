/*
 * func-name: StartAddress
 * func-address: 0x140002e08
 * callers: 0x140002e90
 * callees: 0x1400018d4, 0x1400073d8
 */

__int64 __fastcall StartAddress(wchar_t *lpThreadParameter)
{
  HMODULE v2; // rax
  FARPROC DeviceProperties_RunDLLA; // rbx
  CHAR v5[520]; // [rsp+20h] [rbp-208h] BYREF

  v2 = (HMODULE)sub_1400073D8(L"devmgr.dll");
  if ( !v2 )
    return 0;
  DeviceProperties_RunDLLA = GetProcAddress(v2, "DeviceProperties_RunDLLA");
  if ( !DeviceProperties_RunDLLA )
    return 0;
  wsprintfA(v5, "/MachineName \"\" /DeviceID %S", lpThreadParameter);
  ((void (__fastcall *)(_QWORD, HINSTANCE, CHAR *, __int64))DeviceProperties_RunDLLA)(0, hInstance, v5, 5);
  sub_1400018D4(lpThreadParameter);
  return 1;
}
