/*
 * func-name: sub_140001110
 * func-address: 0x140001110
 * callers: 0x140008320, 0x140009100
 * callees: none
 */

__int64 __fastcall sub_140001110(__int64 a1, int a2, int a3)
{
  if ( a2 >= *(_DWORD *)(a1 + 16) )
    return 0;
  *(_DWORD *)(*(_QWORD *)a1 + 4LL * a2) = a3;
  return 1;
}
