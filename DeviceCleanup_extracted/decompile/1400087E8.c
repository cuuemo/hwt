/*
 * func-name: sub_1400087E8
 * func-address: 0x1400087e8
 * callers: 0x14000885c
 * callees: none
 */

__int64 __fastcall sub_1400087E8(LPCWSTR lpFileName)
{
  DWORD FileAttributesW; // eax
  char v3; // r11
  __int64 result; // rax

  SetLastError(0);
  FileAttributesW = GetFileAttributesW(lpFileName);
  v3 = FileAttributesW;
  if ( FileAttributesW == -1 )
    return 0;
  result = 1;
  if ( (v3 & 1) != 1 )
    return 0;
  return result;
}
