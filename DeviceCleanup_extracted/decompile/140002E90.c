/*
 * func-name: sub_140002E90
 * func-address: 0x140002e90
 * callers: 0x140003cec, 0x14000453c
 * callees: 0x140001854, 0x140001898, 0x1400018d4, 0x140001d84, 0x140006128
 */

void __fastcall sub_140002E90(int a1)
{
  __int64 v2; // rbx
  __int64 v3; // rdi
  void *v4; // rsi
  CONFIGRET Device_IDW; // eax
  HANDLE v6; // rax
  CHAR *v7; // rax
  struct _SP_DEVINFO_DATA DeviceInfoData; // [rsp+30h] [rbp-878h] BYREF
  int lParam; // [rsp+50h] [rbp-858h] BYREF
  _DWORD lParam_4[19]; // [rsp+54h] [rbp-854h] BYREF
  WCHAR v11[1024]; // [rsp+A0h] [rbp-808h] BYREF
  DWORD ThreadId; // [rsp+8B0h] [rbp+8h] BYREF

  if ( a1 >= 0 )
  {
    sub_140001854(lParam_4, 0, 0x44u);
    lParam_4[0] = a1;
    lParam = 4;
    if ( (unsigned int)SendMessageW(hWnd, 0x104Bu, 0, (LPARAM)&lParam) )
    {
      v2 = qword_140010780;
      v3 = *(_QWORD *)(*(_QWORD *)qword_140010780 + 8LL * lParam_4[9]);
      if ( v3 )
      {
        DeviceInfoData.cbSize = 32;
        sub_140001854(&DeviceInfoData.ClassGuid, 0, 0x1Cu);
        if ( SetupDiEnumDeviceInfo(*(HDEVINFO *)(v2 + 16), *(_DWORD *)(v3 + 2392), &DeviceInfoData) )
        {
          v4 = sub_140001898(0x208u);
          sub_140001854(v4, 0, 0x104u);
          Device_IDW = CM_Get_Device_IDW(DeviceInfoData.DevInst, (PWSTR)v4, 0x104u, 0);
          if ( Device_IDW )
          {
            v7 = sub_140001D84(Device_IDW);
            wsprintfW(v11, L"Error %S\n%s", v7, v3 + 16);
            sub_140006128(hDlg, v11, L"DeviceCleanup", 16);
            sub_1400018D4(v4);
          }
          else
          {
            ThreadId = 0;
            v6 = CreateThread(0, 0, (LPTHREAD_START_ROUTINE)StartAddress, v4, 0, &ThreadId);
            if ( v6 )
              CloseHandle(v6);
          }
        }
      }
    }
  }
}
