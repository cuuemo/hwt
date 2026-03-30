/*
 * func-name: sub_140008820
 * func-address: 0x140008820
 * callers: 0x14000885c
 * callees: none
 */

_BOOL8 __fastcall sub_140008820(LPCWSTR lpFileName)
{
  DWORD LastError; // eax

  SetLastError(0);
  LODWORD(lpFileName) = GetFileAttributesW(lpFileName);
  LastError = GetLastError();
  return (_DWORD)lpFileName != -1 || LastError == 5;
}
