/*
 * func-name: sub_140007EB4
 * func-address: 0x140007eb4
 * callers: 0x140003010
 * callees: 0x140007e00
 */

int __fastcall sub_140007EB4(HWND hWnd)
{
  HMONITOR v2; // rax
  int v3; // ebx
  struct tagRECT Rect; // [rsp+20h] [rbp-38h] BYREF
  POINT pt; // [rsp+30h] [rbp-28h]
  int v7; // [rsp+38h] [rbp-20h]
  int v8; // [rsp+3Ch] [rbp-1Ch]
  int v9; // [rsp+40h] [rbp-18h]
  int v10; // [rsp+44h] [rbp-14h]
  int v11; // [rsp+48h] [rbp-10h]
  int v12; // [rsp+4Ch] [rbp-Ch]

  LODWORD(v2) = GetWindowRect(hWnd, &Rect);
  if ( (_DWORD)v2 )
  {
    pt.x = Rect.left + 30;
    pt.y = Rect.top + 30;
    v8 = Rect.top + 30;
    v9 = Rect.left + 30;
    v7 = Rect.right - 30;
    v10 = Rect.bottom - 30;
    v11 = Rect.right - 30;
    v12 = Rect.bottom - 30;
    v3 = 0;
    while ( 1 )
    {
      v2 = MonitorFromPoint(*(&pt + v3), 0);
      if ( v2 )
        break;
      if ( ++v3 >= 4 )
      {
        LODWORD(v2) = sub_140007E00(hWnd);
        return (int)v2;
      }
    }
  }
  return (int)v2;
}
