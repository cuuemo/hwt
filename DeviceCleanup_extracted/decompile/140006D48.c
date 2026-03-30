/*
 * func-name: sub_140006D48
 * func-address: 0x140006d48
 * callers: 0x140006128
 * callees: none
 */

__int64 __fastcall sub_140006D48(const WCHAR *a1, __int16 a2)
{
  __int64 v4; // rcx
  __int64 result; // rax

  v4 = lstrlenW(a1);
  result = 0;
  if ( v4 )
  {
    if ( a1[v4 - 1] == a2 )
      a1[v4 - 1] = 0;
  }
  return result;
}
