/*
 * func-name: sub_140002924
 * func-address: 0x140002924
 * callers: 0x140004704
 * callees: 0x140001248, 0x140001854, 0x1400018d4
 */

__int64 __fastcall sub_140002924(__int64 a1, int a2)
{
  HWND DlgItem; // rax
  WPARAM i; // r8
  __int64 v4; // rdi
  _DWORD *v5; // rbx
  int v6; // eax
  int lParam; // [rsp+20h] [rbp-1E8h] BYREF
  int lParam_4; // [rsp+24h] [rbp-1E4h] BYREF
  int v10; // [rsp+48h] [rbp-1C0h]
  WCHAR Buffer[204]; // [rsp+70h] [rbp-198h] BYREF

  if ( a2 == 7 )
  {
    lParam = 0;
    sub_140001854(&lParam_4, 0, 0x44u);
    for ( i = -1; ; i = v6 )
    {
      lParam_4 = SendMessageW(hWnd, 0x100Cu, i, 0);
      if ( lParam_4 <= -1 )
        break;
      lParam = 4;
      if ( (unsigned int)SendMessageW(hWnd, 0x104Bu, 0, (LPARAM)&lParam)
        && (v4 = v10, (v5 = *(_DWORD **)(*(_QWORD *)qword_140010780 + 8LL * v10)) != 0)
        && v5[3] == 45
        && CM_Get_Device_IDW(*v5, Buffer, 0xC8u, 0) == 13
        && (unsigned int)SendMessageW(hWnd, 0x1008u, lParam_4, 0) )
      {
        nullsub_1();
        sub_1400018D4(v5);
        *(_QWORD *)(*(_QWORD *)qword_140010780 + 8 * v4) = 0;
        v6 = --lParam_4;
      }
      else
      {
        v6 = lParam_4;
      }
    }
  }
  else if ( a2 == 32772 )
  {
    DlgItem = GetDlgItem(hDlg, 1005);
    SetWindowTextW(DlgItem, L"(Press F5 to refresh)");
  }
  return 1;
}
