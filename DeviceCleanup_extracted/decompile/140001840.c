/*
 * func-name: sub_140001840
 * func-address: 0x140001840
 * callers: 0x140007700, 0x1400078b4
 * callees: none
 */

unsigned __int64 __fastcall sub_140001840(unsigned __int64 a1, unsigned int a2, _DWORD *a3)
{
  unsigned __int64 result; // rax
  unsigned __int64 v4; // rdx

  result = a1 / a2;
  v4 = a1 % a2;
  if ( a3 )
    *a3 = v4;
  return result;
}
