/*
 * func-name: sub_1400074FC
 * func-address: 0x1400074fc
 * callers: 0x1400052dc
 * callees: 0x1400074c0
 */

__int64 sub_1400074FC()
{
  unsigned int v0; // ebx
  HMODULE v1; // rax
  BOOL (__stdcall *SetDefaultDllDirectories)(DWORD); // rax

  v0 = 0;
  v1 = sub_1400074C0();
  SetDefaultDllDirectories = (BOOL (__stdcall *)(DWORD))GetProcAddress(v1, "SetDefaultDllDirectories");
  if ( SetDefaultDllDirectories )
    return ((unsigned int (__fastcall *)(__int64))SetDefaultDllDirectories)(2048);
  return v0;
}
