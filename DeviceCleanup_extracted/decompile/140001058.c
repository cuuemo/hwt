/*
 * func-name: sub_140001058
 * func-address: 0x140001058
 * callers: 0x140008320, 0x140009100
 * callees: 0x140001854, 0x140008274
 */

__int64 __fastcall sub_140001058(__int64 a1)
{
  __int64 v2; // rdi
  int v3; // r10d
  __int64 v4; // r9

  v2 = *(_DWORD *)(a1 + 16) / 8;
  sub_140001854(*(_QWORD *)(a1 + 8), 0, v2);
  v3 = 0;
  if ( *(int *)(a1 + 16) > 0 )
  {
    v4 = 0;
    do
    {
      if ( *(_DWORD *)(v4 + *(_QWORD *)a1) )
        *(_BYTE *)(v3 / 8 + *(_QWORD *)(a1 + 8)) |= 1 << (v3 % 8);
      ++v3;
      v4 += 4;
    }
    while ( v3 < *(_DWORD *)(a1 + 16) );
  }
  return sub_140008274(
           -2147483646,
           L"SYSTEM\\CurrentControlSet\\Control\\COM Name Arbiter",
           L"ComDB",
           *(_QWORD *)(a1 + 8),
           v2);
}
