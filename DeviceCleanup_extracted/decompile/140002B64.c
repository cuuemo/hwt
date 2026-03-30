/*
 * func-name: sub_140002B64
 * func-address: 0x140002b64
 * callers: 0x140003cec
 * callees: 0x140001854, 0x140001d84, 0x140001dc0, 0x140006128, 0x140006264
 */

__int64 __fastcall sub_140002B64(HDEVINFO DeviceInfoSet, PSP_DEVINFO_DATA DeviceInfoData)
{
  unsigned int v2; // edi
  BOOL v5; // ebx
  DWORD DevInst; // r12d
  CONFIGRET Device_IDW; // eax
  const wchar_t *v8; // rdx
  WCHAR *v9; // rax
  CHAR *v10; // rax
  _SP_CLASSINSTALL_HEADER ClassInstallParams; // [rsp+20h] [rbp-C48h] BYREF
  int v13; // [rsp+28h] [rbp-C40h]
  int v14; // [rsp+2Ch] [rbp-C3Ch]
  WCHAR Buffer; // [rsp+30h] [rbp-C38h] BYREF
  _BYTE v16[526]; // [rsp+32h] [rbp-C36h] BYREF
  CHAR v17[512]; // [rsp+240h] [rbp-A28h] BYREF
  WCHAR v18[1024]; // [rsp+440h] [rbp-828h] BYREF
  DEVNODE pdnDevInst; // [rsp+C80h] [rbp+18h] BYREF

  v2 = 1;
  ClassInstallParams.cbSize = 8;
  ClassInstallParams.InstallFunction = 5;
  v13 = 1;
  v14 = 0;
  v5 = 0;
  if ( !SetupDiSetClassInstallParamsW(DeviceInfoSet, DeviceInfoData, &ClassInstallParams, 0x10u) )
    return 0;
  Buffer = 0;
  sub_140001854(v16, 0, 0x206u);
  DevInst = DeviceInfoData->DevInst;
  Device_IDW = CM_Get_Device_IDW(DevInst, &Buffer, 0x104u, 0);
  if ( Device_IDW )
  {
    v10 = sub_140001D84(Device_IDW);
    wsprintfA(v17, "Error %s when uninstalling device", v10);
    sub_140006264(hDlg, v17, "Error", 16);
  }
  else
  {
    v5 = SetupDiCallClassInstaller(5u, DeviceInfoSet, DeviceInfoData);
    if ( v5 )
    {
      Buffer = 0;
      if ( !CM_Get_Device_IDW(DevInst, &Buffer, 0x104u, 0) )
      {
        pdnDevInst = 0;
        if ( !CM_Locate_DevNodeW(&pdnDevInst, &Buffer, 0) )
        {
          v8 = L"Device not removed, admin privileges are required";
          if ( dword_1400107CC )
            v8 = L"Device not removed";
          sub_140006128(hDlg, v8, L"Error", 16);
          v5 = 0;
        }
      }
    }
    else
    {
      v9 = sub_140001DC0();
      wsprintfW(v18, L"Error %s removing\n%s", v9, &Buffer);
      sub_140006128(hDlg, v18, L"DeviceCleanup", 16);
    }
  }
  if ( !v5 )
    return 0;
  return v2;
}
