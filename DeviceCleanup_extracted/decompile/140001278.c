/*
 * func-name: sub_140001278
 * func-address: 0x140001278
 * callers: 0x1400012f4, 0x1400012fc
 * callees: 0x1400018d4
 */

void __fastcall sub_140001278(__int64 *a1)
{
  int v2; // esi
  __int64 v3; // rdi
  __int64 v4; // rcx
  void *v5; // rcx

  if ( *a1 )
  {
    v2 = 0;
    if ( *((int *)a1 + 2) > 0 )
    {
      v3 = 0;
      do
      {
        if ( *(_QWORD *)(v3 + *a1) )
        {
          ((void (*)(void))sub_1400018D4)();
          *(_QWORD *)(v3 + *a1) = 0;
        }
        ++v2;
        v3 += 8;
      }
      while ( v2 < *((_DWORD *)a1 + 2) );
    }
    v4 = *a1;
    *((_DWORD *)a1 + 2) = 0;
    sub_1400018D4(v4);
  }
  v5 = (void *)a1[2];
  if ( v5 )
  {
    SetupDiDestroyDeviceInfoList(v5);
    a1[2] = 0;
  }
}
