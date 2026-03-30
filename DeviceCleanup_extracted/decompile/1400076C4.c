/*
 * func-name: sub_1400076C4
 * func-address: 0x1400076c4
 * callers: 0x1400012fc
 * callees: 0x140001878
 */

_BYTE *sub_1400076C4()
{
  _SYSTEMTIME SystemTime; // [rsp+20h] [rbp-18h] BYREF
  struct _FILETIME FileTime; // [rsp+40h] [rbp+8h] BYREF

  GetSystemTime(&SystemTime);
  SystemTimeToFileTime(&SystemTime, &FileTime);
  return sub_140001878(&qword_140016FC8, (__int64)&FileTime, 8);
}
