/*
 * func-name: sub_140004704
 * func-address: 0x140004704
 * callers: 0x1400047fc
 * callees: 0x140001e64, 0x1400026cc, 0x140002924, 0x140003c1c, 0x140003cec, 0x140004534, 0x14000453c
 */

INT_PTR __fastcall sub_140004704(HWND a1, int a2, __int64 a3, __int64 a4)
{
  int v4; // edx
  int v5; // edx
  int v6; // edx
  int v7; // edx
  int v8; // edx
  int v9; // edx
  int v10; // edx
  int v11; // edx

  v4 = a2 - 5;
  if ( v4 )
  {
    v5 = v4 - 11;
    if ( v5 )
    {
      v6 = v5 - 20;
      if ( v6 )
      {
        v7 = v6 - 42;
        if ( !v7 )
          return sub_14000453C(a1, a3, a4);
        v8 = v7 - 45;
        if ( v8 )
        {
          v9 = v8 - 133;
          if ( v9 )
          {
            v10 = v9 - 16;
            if ( !v10 )
              return (unsigned int)sub_140001E64(a1);
            v11 = v10 - 1;
            if ( v11 )
            {
              if ( v11 == 264 )
                sub_140002924((__int64)a1, a3);
            }
            else
            {
              sub_140003CEC(a1, (unsigned __int16)a3);
            }
          }
          else if ( a3 == 13 )
          {
            Sleep(1u);
          }
        }
        else
        {
          sub_1400026CC(a1, a3, (__int16)a4, SWORD1(a4));
        }
      }
      else
      {
        *(_DWORD *)(a4 + 24) = 300;
        *(_DWORD *)(a4 + 28) = 200;
      }
    }
    else
    {
      sub_140003C1C(a1);
    }
  }
  else
  {
    sub_140004534();
  }
  return 0;
}
