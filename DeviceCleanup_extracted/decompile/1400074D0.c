/*
 * func-name: sub_1400074D0
 * func-address: 0x1400074d0
 * callers: 0x1400052dc
 * callees: 0x1400074c0
 */

FARPROC sub_1400074D0()
{
  HMODULE v0; // rax
  FARPROC result; // rax

  v0 = sub_1400074C0();
  result = GetProcAddress(v0, "SetDllDirectoryA");
  if ( result )
    return (FARPROC)((__int64 (__fastcall *)(void *))result)(&unk_14000C29D);
  return result;
}
