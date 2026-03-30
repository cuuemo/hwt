/*
 * func-name: sub_140003CEC
 * func-address: 0x140003cec
 * callers: 0x140004704
 * callees: 0x140001248, 0x14000125c, 0x140001854, 0x1400018d4, 0x140001e04, 0x140002008, 0x14000242c, 0x14000246c, 0x1400024d4, 0x140002534, 0x140002b18, 0x140002b64, 0x140002d38, 0x140002e90, 0x1400036d4, 0x140003bcc, 0x140006128, 0x140006850, 0x140006f20, 0x140007b74, 0x140007c48, 0x140007f44, 0x140008320, 0x140009100
 */

void __fastcall sub_140003CEC(HWND hWnd, int a2)
{
  int v4; // ebx
  int v5; // ebx
  int v6; // ebx
  UINT v7; // ecx
  UINT v8; // ecx
  BOOL v9; // edx
  BOOL v10; // edx
  int v11; // ebx
  int v12; // ebx
  BOOL v13; // edi
  int v14; // ebx
  int v15; // ebx
  int v16; // ebx
  BOOL v17; // edi
  int v18; // ebx
  const wchar_t *v19; // r9
  int v20; // ebx
  int v21; // ebx
  int v22; // ebx
  int v23; // ebx
  int v24; // ebx
  int v25; // eax
  int v26; // eax
  int v27; // ebx
  int v28; // ebx
  unsigned int v29; // r14d
  unsigned int v30; // r15d
  int v31; // eax
  __int64 v32; // r12
  __int64 v33; // rbx
  int v34; // r11d
  __int64 v35; // rdx
  int v36; // r8d
  __int64 v37; // rcx
  unsigned int v38; // eax
  _QWORD *v39; // rdx
  void *v40; // rbx
  int lParam; // [rsp+30h] [rbp-8D8h] BYREF
  _DWORD lParam_4[5]; // [rsp+34h] [rbp-8D4h] BYREF
  WCHAR *p_ApplicationName; // [rsp+48h] [rbp-8C0h]
  int v44; // [rsp+50h] [rbp-8B8h]
  int v45; // [rsp+58h] [rbp-8B0h]
  struct _SP_DEVINFO_DATA DeviceInfoData; // [rsp+80h] [rbp-888h] BYREF
  LPARAM v47; // [rsp+A0h] [rbp-868h] BYREF
  int v48; // [rsp+ACh] [rbp-85Ch]
  int v49; // [rsp+B0h] [rbp-858h]
  WCHAR ApplicationName; // [rsp+F0h] [rbp-818h] BYREF
  char v51[2046]; // [rsp+F2h] [rbp-816h] BYREF

  UpdateWindow(hWnd);
  if ( a2 <= 40111 )
  {
    if ( a2 != 40111 )
    {
      if ( a2 > 40100 )
      {
        v4 = a2 - 40101;
        if ( !v4 )
          goto LABEL_21;
        v5 = v4 - 1;
        if ( !v5 )
        {
          sub_140002D38((LPCWSTR)0x7F02, 0);
          sub_140006850();
          goto LABEL_23;
        }
        v6 = v5 - 2;
        if ( !v6 )
        {
          PostMessageW(hWnd, 0x10u, 0, 0);
          return;
        }
        if ( v6 != 6 )
          return;
        goto LABEL_18;
      }
      if ( a2 == 40100 )
        goto LABEL_12;
      if ( a2 == 1002 )
      {
LABEL_18:
        lParam_4[3] = 2;
        lParam_4[2] = 2;
        SendMessageW(::hWnd, 0x102Bu, 0xFFFFFFFFFFFFFFFFuLL, (LPARAM)&lParam);
        SetFocus(::hWnd);
        sub_14000246C();
        return;
      }
      if ( a2 != 40001 )
      {
        if ( a2 != 40002 )
        {
          if ( a2 != 40010 )
          {
            if ( a2 <= 40019 || a2 > 40021 )
              return;
            goto LABEL_63;
          }
LABEL_12:
          if ( !dword_14001078C )
          {
LABEL_13:
            sub_1400036D4();
            return;
          }
LABEL_21:
          sub_140002D38((LPCWSTR)0x7F02, 0);
          sub_140002008();
          if ( (unsigned int)sub_140007B74(&Parameters) )
            PostMessageW(hWnd, 0x10u, 0, 0);
          goto LABEL_23;
        }
        goto LABEL_18;
      }
    }
LABEL_72:
    sub_140002D38((LPCWSTR)0x7F02, 0);
    v29 = 0;
    lParam = 0;
    v30 = SendMessageW(::hWnd, 0x1032u, 0, 0);
    sub_140001854(lParam_4, 0, 0x44u);
    v31 = SendMessageW(::hWnd, 0x100Cu, 0xFFFFFFFFFFFFFFFFuLL, 2);
    for ( lParam_4[0] = v31; v31 != -1; lParam_4[0] = v31 )
    {
      ++v29;
      dword_14000F45C = v31;
      lParam = 4;
      if ( (unsigned int)SendMessageW(::hWnd, 0x104Bu, 0, (LPARAM)&lParam) )
      {
        SendMessageW(::hWnd, 0x1013u, lParam_4[0], 0);
        v32 = v45;
        v33 = sub_14000125C((__int64 *)qword_140010780, v45);
        if ( v33 )
        {
          DeviceInfoData.cbSize = 32;
          sub_140001854(&DeviceInfoData.ClassGuid, 0, 0x1Cu);
          if ( SetupDiEnumDeviceInfo(*(HDEVINFO *)(qword_140010780 + 16), *(_DWORD *)(v33 + 2392), &DeviceInfoData) )
          {
            wsprintfW(&ApplicationName, L"Removing device %i of %i: \"%s\" (%s)", v29, v30, v33 + 1084, v33 + 16);
            sub_1400024D4((LPARAM)&ApplicationName);
            if ( !(unsigned int)sub_140002B64(*(HDEVINFO *)(qword_140010780 + 16), &DeviceInfoData) )
              break;
            SendMessageW(::hWnd, 0x1008u, lParam_4[0], 0);
            v34 = *(_DWORD *)(v33 + 4);
            if ( v34 > 0 )
            {
              v35 = 0;
              v36 = *(_DWORD *)(qword_140010780 + 8);
              if ( v36 <= 0 )
              {
LABEL_84:
                sub_140009100((unsigned int)v34, v35);
              }
              else
              {
                v37 = *(_QWORD *)qword_140010780;
                while ( !*(_QWORD *)v37 || *(_QWORD *)v37 == v33 || *(_DWORD *)(*(_QWORD *)v37 + 4LL) != v34 )
                {
                  v35 = (unsigned int)(v35 + 1);
                  v37 += 8;
                  if ( (int)v35 >= v36 )
                    goto LABEL_84;
                }
                if ( dword_140010668 >= 10 )
                  sub_140008320((unsigned int)v34, v35);
              }
            }
          }
          v38 = SendMessageW(::hWnd, 0x1004u, 0, 0);
          sub_14000242C(v38);
          UpdateWindow(hDlg);
          sub_140002B18();
          if ( dword_14001078C )
          {
            PostQuitMessage(255);
            return;
          }
          v39 = (_QWORD *)qword_140010780;
          v40 = *(void **)(*(_QWORD *)qword_140010780 + 8 * v32);
          if ( v40 )
          {
            nullsub_1();
            sub_1400018D4(v40);
            v39 = (_QWORD *)qword_140010780;
          }
          *(_QWORD *)(*v39 + 8 * v32) = 0;
        }
      }
      v31 = SendMessageW(::hWnd, 0x100Cu, 0xFFFFFFFFFFFFFFFFuLL, 2);
    }
    v49 = 3;
    v48 = 3;
    SendMessageW(::hWnd, 0x102Bu, dword_14000F45C, (LPARAM)&v47);
    sub_14000246C();
LABEL_23:
    sub_140002D38((LPCWSTR)0x7F00, 0);
    return;
  }
  if ( a2 <= 40503 )
  {
    if ( a2 >= 40500 )
    {
      v18 = a2 - 40500;
      v19 = L"calc.exe";
      if ( v18 && (v20 = v18 - 1) != 0 )
      {
        v21 = v20 - 1;
        if ( v21 )
        {
          if ( v21 == 1 )
            v19 = L"control.exe";
        }
        else
        {
          v19 = L"eventvwr.exe";
        }
      }
      else
      {
        v19 = L"mmc.exe";
      }
      wsprintfW(&ApplicationName, L"%s\\%s", &PathName, v19);
      sub_140007C48(&ApplicationName);
      return;
    }
    v7 = 40200;
    if ( a2 < 40200 )
      return;
    if ( a2 > 40203 )
    {
      if ( a2 <= 40300 )
        return;
      v8 = 40303;
      if ( a2 <= 40303 )
      {
        v11 = a2 - 40301;
        if ( v11 )
        {
          v12 = v11 - 1;
          if ( v12 )
          {
            if ( v12 != 1 )
              goto LABEL_13;
            v13 = dword_1400107C8 == 0;
            dword_1400107C8 = v13;
          }
          else
          {
            v8 = 40302;
            v13 = dword_1400107C4 == 0;
            dword_1400107C4 = v13;
          }
        }
        else
        {
          v8 = 40301;
          v13 = dword_1400107C0 == 0;
          dword_1400107C0 = v13;
        }
        sub_140001E04(v8, v13);
        goto LABEL_13;
      }
      if ( a2 == 40400 )
      {
        v10 = dword_14000F458 == 0;
        dword_14000F458 = v10;
        sub_140001E04(0x9DD0u, v10);
        sub_140003BCC();
      }
      else if ( a2 == 40401 )
      {
        v9 = dword_140010788 == 0;
        dword_140010788 = v9;
        sub_140001E04(0x9DD1u, v9);
        sub_140006F20(hDlg, (dword_140010788 != 0) - 2LL);
      }
      return;
    }
    v14 = a2 - 40200;
    if ( v14 )
    {
      v15 = v14 - 1;
      if ( v15 )
      {
        v16 = v15 - 1;
        if ( v16 )
        {
          if ( v16 != 1 )
          {
LABEL_50:
            sub_140002534();
            goto LABEL_13;
          }
          v7 = 40203;
          v17 = dword_140010794 == 0;
          dword_140010794 = v17;
        }
        else
        {
          v7 = 40202;
          v17 = dword_1400107A0 == 0;
          dword_1400107A0 = v17;
        }
      }
      else
      {
        v7 = 40201;
        v17 = dword_140010798 == 0;
        dword_140010798 = v17;
      }
    }
    else
    {
      v17 = dword_14001079C == 0;
      dword_14001079C = v17;
    }
    sub_140001E04(v7, v17);
    goto LABEL_50;
  }
  v22 = a2 - 40900;
  if ( !v22 )
  {
    DialogBoxParamW(hInstance, (LPCWSTR)0x68, hDlg, (DLGPROC)DialogFunc, 0);
    return;
  }
  v23 = v22 - 101;
  if ( !v23 )
    goto LABEL_72;
  v24 = v23 - 1;
  if ( v24 )
  {
    if ( v24 != 1 )
      return;
LABEL_63:
    v25 = SendMessageW(::hWnd, 0x1032u, 0, 0);
    if ( v25 > 10 )
    {
      wsprintfW(&ApplicationName, L"This opens %i property dialogs! Sure?", (unsigned int)v25);
      if ( (unsigned int)sub_140006128(hDlg, &ApplicationName, L"DeviceCleanup", 36) != 6 )
        return;
      SetForegroundWindow(hDlg);
    }
    sub_140002D38((LPCWSTR)0x7F8A, 0);
    v26 = SendMessageW(::hWnd, 0x100Cu, 0xFFFFFFFFFFFFFFFFuLL, 2);
    v27 = v26;
    while ( v26 != -1 )
    {
      sub_140002E90(v27);
      v26 = SendMessageW(::hWnd, 0x100Cu, v27, 2);
      v27 = v26;
    }
    Sleep(0x1F4u);
    goto LABEL_23;
  }
  v28 = dword_14000F460;
  if ( dword_14000F460 > -1 )
  {
    ApplicationName = 0;
    sub_140001854(v51, 0, 0x1FEu);
    lParam_4[1] = v28;
    v44 = 256;
    p_ApplicationName = &ApplicationName;
    SendMessageW(::hWnd, 0x1073u, dword_14000F45C, (LPARAM)&lParam);
    sub_140007F44(&ApplicationName);
  }
}
