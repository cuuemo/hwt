/*
 * func-name: sub_1400010FC
 * func-address: 0x1400010fc
 * callers: 0x140008320, 0x140009100
 * callees: none
 */

__int64 __fastcall sub_1400010FC(__int64 a1, int a2)
{
  if ( a2 >= *(_DWORD *)(a1 + 16) )
    return 0;
  else
    return *(unsigned int *)(*(_QWORD *)a1 + 4LL * a2);
}
