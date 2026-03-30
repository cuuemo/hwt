/*
 * func-name: sub_14000453C
 * func-address: 0x14000453c
 * callers: 0x140004704
 * callees: 0x140002d38, 0x140002e90, 0x140007a78
 */

__int64 __fastcall sub_14000453C(HWND a1, int a2, __int64 a3)
{
  unsigned int v4; // eax
  int v5; // ebx
  int v6; // eax
  int v7; // ebx
  __int64 v8; // rdx
  __int64 v9; // r8

  if ( a2 == 1000 )
  {
    v4 = *(_DWORD *)(a3 + 16);
    if ( v4 == -108 )
    {
      if ( dword_140010790 < 10 && *(int *)(a3 + 28) < 10 )
      {
        if ( dword_140010790 == *(_DWORD *)(a3 + 28) )
          dword_14000F478[*(int *)(a3 + 28)] = dword_14000F478[*(int *)(a3 + 28)] == 0;
        SendMessageW(hWnd, 0x1051u, *(int *)(a3 + 28), (LPARAM)sub_140003584);
        v8 = *(int *)(a3 + 28);
        v9 = (unsigned int)dword_14000F478[v8];
        dword_140010790 = *(_DWORD *)(a3 + 28);
        sub_140007A78(hWnd, v8, v9);
      }
    }
    else if ( v4 == -101 )
    {
      SetTimer(a1, 0x190969u, 0xAu, (TIMERPROC)TimerFunc);
    }
    else if ( v4 != -15 )
    {
      if ( v4 == -5 )
      {
        if ( (unsigned int)SendMessageW(hWnd, 0x1032u, 0, 0) <= 1 )
        {
          v6 = SendMessageW(hWnd, 0x100Cu, 0xFFFFFFFFFFFFFFFFuLL, 1);
          v7 = v6;
          if ( v6 != -1 )
          {
            SendMessageW(hWnd, 0x1043u, 0, v6);
            dword_14000F45C = v7;
          }
        }
      }
      else if ( v4 > 0xFFFFFFFB )
      {
        if ( v4 <= 0xFFFFFFFD )
        {
          v5 = SendMessageW(hWnd, 0x100Cu, 0xFFFFFFFFFFFFFFFFuLL, 1);
          if ( v5 != -1 )
          {
            sub_140002D38((LPCWSTR)0x7F02, 0);
            sub_140002E90(v5);
            sub_140002D38((LPCWSTR)0x7F00, 0);
          }
        }
        else if ( v4 == -2 )
        {
          dword_14000F45C = SendMessageW(hWnd, 0x100Cu, 0xFFFFFFFFFFFFFFFFuLL, 1);
        }
      }
    }
  }
  return 0;
}
