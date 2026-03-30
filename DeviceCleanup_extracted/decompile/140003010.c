/*
 * func-name: DialogFunc
 * func-address: 0x140003010
 * callers: 0x140003cec
 * callees: 0x140002d38, 0x140007eb4, 0x1400086f4, 0x140008aec, 0x140008fc8
 */

HGDIOBJ __fastcall DialogFunc(HWND a1, int a2, HDC a3, HWND a4)
{
  int v4; // edx
  int v7; // edx
  int v8; // edx
  int v9; // edx
  int v10; // edx
  int v11; // edx
  __int64 v12; // rcx
  HWND Parent; // rax
  void *v15; // rax
  HDC DC; // rbx
  int v17; // eax
  HWND DlgItem; // rax
  int cy; // esi
  HANDLE ImageW; // rdi
  HWND v21; // rbx
  struct tagPOINT Point; // [rsp+40h] [rbp-128h] BYREF
  struct tagRECT Rect; // [rsp+48h] [rbp-120h] BYREF
  struct tagRECT v24; // [rsp+58h] [rbp-110h] BYREF
  struct tagRECT v25; // [rsp+68h] [rbp-100h] BYREF
  LOGFONTW pv; // [rsp+80h] [rbp-E8h] BYREF
  WCHAR String[64]; // [rsp+E0h] [rbp-88h] BYREF

  v4 = a2 - 16;
  if ( !v4 )
  {
    EndDialog(a1, 0);
    DeleteObject(ho);
    DestroyWindow(qword_140010A10);
    goto LABEL_18;
  }
  v7 = v4 - 6;
  if ( v7 )
  {
    v8 = v7 - 250;
    if ( v8 )
    {
      v9 = v8 - 1;
      if ( !v9 )
      {
        if ( (_WORD)a3 == 1 )
        {
          sub_140002D38((LPCWSTR)0x7F00, 0);
          EndDialog(a1, 0);
        }
        return 0;
      }
      v10 = v9 - 39;
      if ( v10 )
      {
        v11 = v10 - 200;
        if ( v11 )
        {
          if ( v11 == 1 )
          {
            sub_140002D38((LPCWSTR)0x7F8A, 0);
            sub_140008FC8(L"http://www.uwe-sieber.de");
          }
          return 0;
        }
        Point.x = (__int16)a4;
        Point.y = SWORD1(a4);
        ClientToScreen(a1, &Point);
        if ( PtInRect(&::Rect, Point) )
        {
          v12 = 32649;
LABEL_19:
          sub_140002D38((LPCWSTR)v12, 0);
          return 0;
        }
LABEL_18:
        v12 = 32512;
        goto LABEL_19;
      }
      if ( a4 == qword_140010A20 )
      {
        SetTextColor(a3, 0xFF0000u);
        SetBkMode(a3, 1);
        SelectObject(a3, ho);
        return GetStockObject(5);
      }
    }
    else
    {
      GetWindowRect(a1, &Rect);
      Parent = GetParent(a1);
      GetWindowRect(Parent, &v24);
      MoveWindow(
        a1,
        v24.left + (Rect.left + v24.right - v24.left - Rect.right) / 2,
        v24.top + (Rect.top + v24.bottom - v24.top - Rect.bottom) / 2,
        Rect.right - Rect.left,
        Rect.bottom - Rect.top,
        0);
      sub_140007EB4(a1);
      wsprintfW(String, L"Device Cleanup Tool (%s)", &unk_140010694);
      SetDlgItemTextW(a1, 1101, String);
      wsprintfW(String, L"Version %s", &unk_1400106CC);
      SetDlgItemTextW(a1, 1102, String);
      v15 = (void *)SendMessageW(a1, 0x31u, 0, 0);
      GetObjectW(v15, 92, &pv);
      pv.lfUnderline = 1;
      ho = CreateFontIndirectW(&pv);
      qword_140010A20 = GetDlgItem(a1, 1103);
      DC = GetDC(qword_140010A20);
      SelectObject(DC, ho);
      v17 = lstrlenW(L"www.uwe-sieber.de");
      GetTextExtentPoint32W(DC, L"www.uwe-sieber.de", v17, (LPSIZE)&Point);
      ReleaseDC(qword_140010A20, DC);
      SetWindowPos(qword_140010A20, 0, 0, 0, Point.x, Point.y + 1, 0x16u);
      DlgItem = GetDlgItem(a1, 1103);
      GetWindowRect(DlgItem, &::Rect);
      cy = sub_1400086F4(32);
      ImageW = LoadImageW(hInstance, (LPCWSTR)0x64, 1u, cy, cy, 0x8000u);
      v21 = GetDlgItem(a1, 1104);
      GetWindowRect(v21, &v25);
      SetWindowPos(v21, 0, 0, 0, cy, cy, 6u);
      SendDlgItemMessageW(a1, 1104, 0x170u, (WPARAM)ImageW, 0);
      qword_140010A10 = (HWND)sub_140008AEC(1104, a1, &Parameters);
    }
  }
  else
  {
    PostMessageW(a1, 0x10u, 0, 0);
  }
  return 0;
}
