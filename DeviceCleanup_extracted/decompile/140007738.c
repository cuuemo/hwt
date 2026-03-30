/*
 * func-name: sub_140007738
 * func-address: 0x140007738
 * callers: 0x1400012fc
 * callees: 0x14000179c, 0x140007638, 0x140007700
 */

__int64 __fastcall sub_140007738(__int64 a1, signed __int64 *a2)
{
  signed __int64 v4; // rax
  WCHAR v6[268]; // [rsp+20h] [rbp-218h] BYREF
  struct _FILETIME v7; // [rsp+250h] [rbp+18h] BYREF

  sub_14000179C();
  wsprintfW(v6, L"%s\\%s", L"System\\CurrentControlSet\\Enum", a1);
  if ( !(unsigned int)sub_140007638(HKEY_LOCAL_MACHINE, v6, &v7) )
  {
    v4 = sub_140007700((__int64)&v7);
    if ( v4 > -1 )
      *a2 = v4;
  }
  sub_14000179C();
  return 1;
}
