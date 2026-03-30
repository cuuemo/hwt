/*
 * func-name: sub_140005518
 * func-address: 0x140005518
 * callers: 0x140006128
 * callees: 0x140001854, 0x140001878, 0x140001898, 0x1400018d4, 0x1400054c8, 0x140007530, 0x140007f44, 0x140008068, 0x1400083b0, 0x1400083c0, 0x140008400, 0x140008440, 0x140008534, 0x1400086f4, 0x140008a9c, 0x140008ad8
 */

INT_PTR __fastcall sub_140005518(HWND a1, int a2, HDC a3)
{
  int v3; // edx
  HWND v5; // r14
  int v6; // edx
  int v7; // edx
  int v8; // edx
  int v9; // edx
  int v10; // eax
  int v11; // ebx
  HBRUSH SysColorBrush; // rax
  DWORD SysColor; // eax
  DWORD v14; // eax
  WCHAR *v16; // rdi
  HMODULE ModuleHandleA; // rax
  HRESULT (__stdcall *LoadIconMetric)(HINSTANCE, PCWSTR, int, HICON *); // r13
  HWND DlgItem; // rbp
  HWND v20; // r15
  int v21; // edi
  int v22; // eax
  __int64 v23; // rax
  WPARAM IconW; // rax
  const WCHAR *v25; // rax
  HDC DC; // rdi
  HGDIOBJ StockObject; // rax
  HFONT v28; // rax
  int v29; // r13d
  int v30; // ebp
  int v31; // edi
  const WCHAR *v32; // r14
  __int64 v33; // r15
  WCHAR *v34; // rbx
  int v35; // r13d
  int v36; // eax
  int v37; // eax
  int v38; // edi
  __int64 v39; // rdx
  int v40; // edi
  const WCHAR *v41; // r8
  int v42; // ecx
  __int64 v43; // rbx
  __int64 v44; // rbp
  WCHAR *v45; // r15
  __int64 v46; // rcx
  WCHAR *v47; // rdi
  int v48; // eax
  int v49; // eax
  WCHAR *v50; // rdi
  __int64 i; // rcx
  WCHAR v52; // ax
  int v53; // r13d
  LONG v54; // ebp
  LONG right; // edi
  LONG bottom; // r15d
  int v57; // eax
  int v58; // ebx
  int v59; // ebp
  int v60; // esi
  int v61; // edi
  unsigned int v62; // ebx
  unsigned int v63; // eax
  int v64; // edi
  unsigned int v65; // ebx
  unsigned int v66; // eax
  int v67; // r13d
  int v68; // ebp
  int v69; // r13d
  int cy; // edi
  int v71; // eax
  int v72; // edi
  int v73; // eax
  int v74; // eax
  HWND v75; // rdi
  int v76; // [rsp+40h] [rbp-2F8h]
  int v77; // [rsp+40h] [rbp-2F8h]
  int v78; // [rsp+44h] [rbp-2F4h]
  int v79; // [rsp+44h] [rbp-2F4h]
  int v80; // [rsp+48h] [rbp-2F0h]
  int v81; // [rsp+48h] [rbp-2F0h]
  int v82; // [rsp+4Ch] [rbp-2ECh] BYREF
  int v83; // [rsp+50h] [rbp-2E8h] BYREF
  LPCWSTR lpString; // [rsp+58h] [rbp-2E0h]
  HWND hWnd; // [rsp+60h] [rbp-2D8h]
  HDC hdc; // [rsp+68h] [rbp-2D0h]
  WPARAM wParam; // [rsp+70h] [rbp-2C8h] BYREF
  struct tagRECT String2; // [rsp+78h] [rbp-2C0h] BYREF
  HWND v89; // [rsp+88h] [rbp-2B0h]
  __int64 v90; // [rsp+90h] [rbp-2A8h]
  HWND hWndInsertAfter; // [rsp+B8h] [rbp-280h]
  struct tagRECT Rect; // [rsp+C0h] [rbp-278h] BYREF
  struct tagRECT v93; // [rsp+D0h] [rbp-268h] BYREF
  struct tagRECT v94; // [rsp+E0h] [rbp-258h] BYREF
  HWND v95; // [rsp+F0h] [rbp-248h]
  int pvParam; // [rsp+100h] [rbp-238h] BYREF
  _BYTE v97[404]; // [rsp+104h] [rbp-234h] BYREF
  LOGFONTW lf; // [rsp+298h] [rbp-A0h] BYREF
  int v100; // [rsp+348h] [rbp+10h]
  int v101; // [rsp+348h] [rbp+10h]
  LONG v102; // [rsp+348h] [rbp+10h]

  v3 = a2 - 6;
  v5 = a1;
  if ( !v3 )
  {
    if ( a3 == (HDC)1 && qword_140010E40 && qword_140010E40 != GetDesktopWindow() )
      EnableWindow(qword_140010E40, 0);
    return 0;
  }
  v6 = v3 - 10;
  if ( !v6 )
  {
    if ( qword_140010E40 && qword_140010E40 != GetDesktopWindow() )
      EnableWindow(qword_140010E40, 1);
    PostMessageW(v5, 0x12u, 0, 0);
    return 0;
  }
  v7 = v6 - 55;
  if ( !v7 )
  {
    SetWindowPos(a1, HWND_MESSAGE|0x2LL, 0, 0, 0, 0, 0x13u);
    return 0;
  }
  v8 = v7 - 201;
  if ( v8 )
  {
    v9 = v8 - 1;
    if ( v9 )
    {
      if ( v9 == 39 )
      {
        v10 = dword_140016F7C;
        if ( (dword_140016F7C & 1) != 0 )
        {
          v11 = dword_140016F78;
        }
        else
        {
          v11 = 5;
          v10 = dword_140016F7C | 1;
          dword_140016F7C |= 1u;
          if ( dword_140010668 == 5 )
            v11 = 15;
          dword_140016F78 = v11;
        }
        if ( (v10 & 2) == 0 )
        {
          dword_140016F7C = v10 | 2;
          SysColorBrush = GetSysColorBrush(v11);
          v11 = dword_140016F78;
          qword_140016F70 = (__int64)SysColorBrush;
        }
        SysColor = GetSysColor(v11);
        SetBkColor(a3, SysColor);
        v14 = GetSysColor(18);
        SetTextColor(a3, v14);
        return qword_140016F70;
      }
    }
    else if ( (_WORD)a3 )
    {
      if ( (unsigned __int16)a3 > 2u )
      {
        if ( (unsigned __int16)a3 == 40003 )
        {
          v16 = (WCHAR *)sub_140001898(0x2000u);
          sub_140001878(&String2, (__int64)L"---------------------------\r\n", 60);
          lstrcpyW(v16, (LPCWSTR)&String2);
          lstrcatW(v16, &word_140010E50);
          lstrcatW(v16, L"\r\n");
          lstrcatW(v16, (LPCWSTR)&String2);
          lstrcatW(v16, word_140012F50);
          lstrcatW(v16, L"\r\n");
          lstrcatW(v16, (LPCWSTR)&String2);
          lstrcatW(v16, L"OK\r\n");
          lstrcatW(v16, (LPCWSTR)&String2);
          sub_140007F44(v16);
          sub_1400018D4(v16);
          return 0;
        }
        if ( (unsigned __int16)a3 <= 0x9C53u || (unsigned __int16)a3 > 0x9C55u )
          return 0;
      }
      PostMessageW(a1, 0x10u, 0, 0);
      return (unsigned __int16)a3;
    }
    return 0;
  }
  ModuleHandleA = GetModuleHandleA("Comctl32.dll");
  LoadIconMetric = (HRESULT (__stdcall *)(HINSTANCE, PCWSTR, int, HICON *))GetProcAddress(
                                                                             ModuleHandleA,
                                                                             "LoadIconMetric");
  DlgItem = GetDlgItem(v5, 2001);
  hWnd = DlgItem;
  hWndInsertAfter = GetDlgItem(v5, 2002);
  *(_QWORD *)&String2.left = DlgItem;
  *(_QWORD *)&String2.right = GetDlgItem(v5, 2003);
  v20 = GetDlgItem(v5, 1);
  v95 = v20;
  v89 = v20;
  v90 = 0;
  sub_140008068(v5);
  SetWindowPos(DlgItem, hWndInsertAfter, 0, 0, 0, 0, 0x13u);
  SetWindowPos(*(HWND *)&String2.right, hWndInsertAfter, 0, 0, 0, 0, 0x13u);
  v21 = sub_1400086F4(20);
  v22 = sub_1400086F4(20);
  SetWindowPos(*(HWND *)&String2.right, 0, v22, v21, ::cy, ::cy, 4u);
  wParam = 0;
  if ( !LoadIconMetric
    || (v23 = sub_1400054C8(dword_140014F50),
        ((void (__fastcall *)(_QWORD, __int64, __int64, WPARAM *))LoadIconMetric)(0, v23, 1, &wParam),
        (IconW = wParam) == 0) )
  {
    v25 = (const WCHAR *)sub_1400054C8(dword_140014F50);
    IconW = (WPARAM)LoadIconW(0, v25);
    wParam = IconW;
  }
  SendDlgItemMessageW(v5, 2003, 0x170u, IconW, 0);
  SetWindowTextW(v5, &word_140010E50);
  DC = GetDC(DlgItem);
  hdc = DC;
  SetMapMode(DC, 1);
  StockObject = GetStockObject(17);
  dword_140014F54 = 0;
  ::wParam = StockObject;
  pvParam = 500;
  sub_140001854(v97, 0, 0x1F0u);
  if ( SystemParametersInfoW(0x29u, 0x1F4u, &pvParam, 0)
    || (pvParam -= 4, SystemParametersInfoW(0x29u, 0x1F4u, &pvParam, 0)) )
  {
    v28 = CreateFontIndirectW(&lf);
    dword_140014F54 = 1;
    ::wParam = v28;
  }
  else
  {
    v28 = (HFONT)::wParam;
  }
  SelectObject(DC, v28);
  SendMessageW(v20, 0x30u, (WPARAM)::wParam, 0);
  SendMessageW(DlgItem, 0x30u, (WPARAM)::wParam, 0);
  v29 = sub_140008A9C(DC, "\t");
  v76 = v29;
  v80 = sub_140008A9C(DC, " ");
  sub_140001854(chText, 0, 0x2000u);
  lpString = chText;
  v30 = 0;
  v31 = 0;
  v100 = 0;
  *(_QWORD *)&String2.left = 0;
  if ( word_140010F50[0] )
  {
    v32 = lpString;
    v33 = 0;
    v34 = word_140010F50;
    v35 = 1;
    do
    {
      if ( *v34 == 9 )
      {
        if ( ++v30 == 1 )
        {
          v36 = sub_140008A9C(hdc, v32);
          if ( v36 > v31 )
            v31 = v36;
          v37 = lstrlenW(v32);
          if ( v37 > v100 )
            v100 = v37;
        }
      }
      else if ( *v34 == 10 )
      {
        v30 = 0;
        v32 = &chText[v35];
      }
      ++v35;
      chText[v33] = *v34;
      v33 = ++*(_QWORD *)&String2.left;
      v34 = &word_140010F50[*(_QWORD *)&String2.left];
    }
    while ( *v34 );
    v5 = a1;
    v29 = v76;
  }
  v38 = sub_1400086F4(8) + v31;
  if ( v38 < (int)sub_1400086F4(32) )
    v38 = sub_1400086F4(32);
  v77 = v29 / 2 + v29 * (v29 / v38 + sub_1400086F4(1));
  v78 = v100 + 2;
  sub_140001854(chText, 0, 0x2000u);
  sub_140001854(word_140012F50, 0, 0x2000u);
  v39 = 0;
  v40 = 0;
  v101 = 0;
  v41 = chText;
  lpString = chText;
  *(_QWORD *)&String2.left = 0;
  v42 = 0;
  v43 = 0;
  v44 = 0;
  if ( word_140010F50[0] )
  {
    v45 = word_140010F50;
    do
    {
      if ( *v45 == 9 )
      {
        if ( ++v42 == 1 )
        {
          v46 = (unsigned int)(v78 - lstrlenW(v41));
          if ( (int)v46 > 0 )
          {
            v47 = &word_140012F50[v43];
            v43 += (unsigned int)v46;
            while ( v46 )
            {
              *v47++ = 32;
              --v46;
            }
            v40 = v101;
          }
          v48 = sub_140008A9C(hdc, lpString);
          v41 = lpString;
          v49 = (v77 - v48) / v80;
          if ( v49 > 0 )
          {
            v50 = &chText[v44];
            for ( i = (unsigned int)v49; i; --i )
              *v50++ = 32;
            v40 = v49 + v101;
            v44 += (unsigned int)v49;
          }
          v42 = 1;
          v39 = *(_QWORD *)&String2.left;
        }
      }
      else if ( *v45 == 10 )
      {
        v41 = &chText[v40 + 1];
        v42 = 0;
        lpString = v41;
        if ( v44 > 0 && chText[v44 - 1] != 13 )
          word_140012F50[v43++] = 13;
      }
      v52 = *v45;
      ++v40;
      chText[v44++] = *v45;
      v101 = v40;
      if ( v52 != 9 )
        word_140012F50[v43++] = v52;
      *(_QWORD *)&String2.left = ++v39;
      v45 = &word_140010F50[v39];
    }
    while ( *v45 );
    v5 = a1;
  }
  v53 = sub_1400086F4(20);
  v54 = sub_1400086F4(32);
  right = sub_1400086F4(40);
  v102 = right;
  bottom = sub_1400086F4(12);
  v57 = sub_1400086F4(13);
  String2.left = 0;
  v58 = v53 + v54 + v57;
  String2.top = 0;
  *(_QWORD *)&String2.right = 0;
  if ( DrawTextExW(hdc, chText, -1, &String2, 0xD40u, 0) )
  {
    right = String2.right;
    bottom = String2.bottom;
    v102 = String2.right;
  }
  if ( right < (int)sub_1400086F4(40) )
  {
    right = sub_1400086F4(40);
    v102 = right;
  }
  if ( GetWindowRect(hWnd, &Rect) )
  {
    sub_140007530(v5, &Rect);
    if ( bottom >= v54 )
    {
      MoveWindow(hWnd, v58, v53, right, bottom, 0);
    }
    else
    {
      MoveWindow(hWnd, v58, v53 + (v54 - bottom) / 2, Rect.right - Rect.left, Rect.bottom - Rect.top, 0);
      bottom = sub_1400086F4(35);
    }
  }
  GetWindowRect(v5, &v93);
  GetClientRect(v5, &v94);
  v59 = v93.right + v94.left - v94.right - v93.left;
  v60 = v93.bottom + v94.top - v94.bottom - v93.top;
  v82 = right + v59 + sub_1400086F4(80);
  v61 = sub_1400086F4(16);
  v62 = sub_140008400() - v61;
  v63 = sub_1400086F4(140);
  sub_1400083B0(&v82, v63, v62);
  v83 = bottom + v60 + sub_1400086F4(72);
  v64 = sub_1400086F4(16);
  v65 = sub_1400083C0() - v64;
  v66 = sub_1400086F4(130);
  sub_1400083B0(&v83, v66, v65);
  v67 = v82 - v59;
  v81 = v83 - v60;
  v68 = sub_1400086F4(82);
  v79 = sub_1400086F4(25);
  if ( dword_140010668 == 5 )
    v69 = (v67 - v68) / 2;
  else
    v69 = v67 - v68 - sub_1400086F4(5);
  cy = bottom + sub_1400086F4(5);
  v71 = sub_1400086F4(20);
  SetWindowPos(hWnd, hWndInsertAfter, 0, 0, v102 + v71, cy, 2u);
  v72 = bottom + sub_1400086F4(34);
  v73 = v102 + sub_1400086F4(100);
  SetWindowPos(hWndInsertAfter, 0, 0, 0, v73, v72, 6u);
  SetWindowPos(v5, 0, 0, 0, v82, v83, 6u);
  v74 = sub_1400086F4(5);
  v75 = v95;
  SetWindowPos(v95, 0, v69, v81 - v79 - v74, v68, v79, 4u);
  SetWindowTextW(hWnd, chText);
  sub_140008440(v5, qword_140010E40);
  sub_140008534(v5);
  sub_140008AD8(v5, v75);
  ShowWindow(v5, 1);
  ReleaseDC(v5, hdc);
  return 1;
}
