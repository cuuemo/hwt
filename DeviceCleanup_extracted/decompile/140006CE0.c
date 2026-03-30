/*
 * func-name: sub_140006CE0
 * func-address: 0x140006ce0
 * callers: 0x14000885c
 * callees: none
 */

__int64 __fastcall sub_140006CE0(const WCHAR *a1, WCHAR a2)
{
  __int64 v4; // rcx
  __int64 result; // rax

  v4 = lstrlenW(a1);
  result = 0;
  if ( !v4 || a1[v4 - 1] != a2 )
  {
    a1[v4] = a2;
    a1[v4 + 1] = 0;
  }
  return result;
}
