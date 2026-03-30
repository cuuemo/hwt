/*
 * func-name: sub_140008718
 * func-address: 0x140008718
 * callers: 0x14000885c
 * callees: none
 */

__int64 __fastcall sub_140008718(LPCWSTR lpString2)
{
  HANDLE FileW; // rax
  HANDLE v3; // rax
  WCHAR String1[268]; // [rsp+40h] [rbp-218h] BYREF

  FileW = CreateFileW(lpString2, 0x40000000u, 7u, 0, 3u, 0, 0);
  if ( FileW == (HANDLE)-1LL )
  {
    if ( GetLastError() != 2 )
      return 0;
    if ( lpString2[1] != 58 )
      return 0;
    lstrcpyW(String1, lpString2);
    PathRenameExtensionW(String1, L".tmp");
    v3 = CreateFileW(String1, 0x40000000u, 7u, 0, 4u, 0x102u, 0);
    if ( v3 == (HANDLE)-1LL )
      return 0;
    CloseHandle(v3);
    DeleteFileW(String1);
  }
  else
  {
    CloseHandle(FileW);
  }
  return 1;
}
