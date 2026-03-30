/*
 * func-name: sub_14000125C
 * func-address: 0x14000125c
 * callers: 0x140003cec
 * callees: none
 */

__int64 __fastcall sub_14000125C(__int64 *a1, int a2)
{
  __int64 v2; // rcx

  if ( a2 < *((_DWORD *)a1 + 2) && (v2 = *a1, *(_QWORD *)(v2 + 8LL * a2)) )
    return *(_QWORD *)(v2 + 8LL * a2);
  else
    return 0;
}
