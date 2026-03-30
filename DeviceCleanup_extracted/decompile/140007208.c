/*
 * func-name: sub_140007208
 * func-address: 0x140007208
 * callers: 0x14000885c
 * callees: none
 */

_FILETIME __fastcall sub_140007208(const WCHAR *a1)
{
  HANDLE FileW; // rax
  void *v2; // rbx
  _FILETIME LastWriteTime; // [rsp+58h] [rbp+10h] BYREF

  LastWriteTime.dwLowDateTime = 0;
  LastWriteTime.dwHighDateTime = 0;
  FileW = CreateFileW(a1, 0, 3u, 0, 3u, 0, 0);
  v2 = FileW;
  if ( FileW != (HANDLE)-1LL )
  {
    GetFileTime(FileW, 0, 0, &LastWriteTime);
    CloseHandle(v2);
  }
  return LastWriteTime;
}
