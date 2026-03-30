/*
 * func-name: sub_1400012FC
 * func-address: 0x1400012fc
 * callers: 0x1400036d4
 * callees: 0x140001220, 0x140001278, 0x140001854, 0x140001898, 0x1400023e4, 0x140002b18, 0x140006d1c, 0x140006f48, 0x1400076c4, 0x140007738, 0x1400077d8
 */

__int64 __fastcall sub_1400012FC(__int64 a1)
{
  HDEVINFO ClassDevsW; // rsi
  DWORD v3; // edi
  DWORD v4; // r13d
  SIZE_T v5; // rax
  int v6; // esi
  DWORD i; // edx
  DWORD v8; // r12d
  CONFIGRET DevNode_Status; // eax
  __int64 v10; // rax
  __int64 v11; // rdi
  PULONG pulLength; // [rsp+20h] [rbp-518h]
  _SP_DEVINFO_DATA DeviceInfoData; // [rsp+30h] [rbp-508h] BYREF
  WCHAR psz1; // [rsp+50h] [rbp-4E8h] BYREF
  WCHAR pszSrc[4]; // [rsp+52h] [rbp-4E6h] BYREF
  __int64 v17; // [rsp+5Ah] [rbp-4DEh]
  __int64 v18; // [rsp+62h] [rbp-4D6h]
  __int16 v19; // [rsp+6Ah] [rbp-4CEh]
  WCHAR String1; // [rsp+70h] [rbp-4C8h] BYREF
  _BYTE v21[110]; // [rsp+72h] [rbp-4C6h] BYREF
  WCHAR Buffer; // [rsp+E0h] [rbp-458h] BYREF
  _BYTE v23[526]; // [rsp+E2h] [rbp-456h] BYREF
  WCHAR v24[292]; // [rsp+2F0h] [rbp-248h] BYREF
  ULONG v25; // [rsp+540h] [rbp+8h] BYREF
  ULONG pulProblemNumber; // [rsp+548h] [rbp+10h] BYREF
  ULONG pulStatus; // [rsp+550h] [rbp+18h] BYREF

  sub_140001278((__int64 *)a1);
  ClassDevsW = SetupDiGetClassDevsW(0, 0, 0, 4u);
  *(_QWORD *)(a1 + 16) = ClassDevsW;
  if ( ClassDevsW != (HDEVINFO)-1LL )
  {
    String1 = 0;
    sub_140001854(v21, 0, 102);
    DeviceInfoData.cbSize = 32;
    sub_140001854(&DeviceInfoData.ClassGuid, 0, 28);
    v3 = 0;
    if ( SetupDiEnumDeviceInfo(ClassDevsW, 0, &DeviceInfoData) )
    {
      do
        ++v3;
      while ( SetupDiEnumDeviceInfo(*(HDEVINFO *)(a1 + 16), v3, &DeviceInfoData) );
      if ( v3 )
      {
        v4 = v3 + 100;
        v5 = 8LL * (int)(v3 + 100);
        if ( !is_mul_ok((int)(v3 + 100), 8u) )
          v5 = -1;
        *(_QWORD *)a1 = sub_140001898(v5);
        GetTickCount();
        sub_1400076C4();
        v6 = 0;
        for ( i = 0; ; i = v8 )
        {
          if ( !SetupDiEnumDeviceInfo(*(HDEVINFO *)(a1 + 16), i, &DeviceInfoData) )
            return *(unsigned int *)(a1 + 8);
          if ( dword_14001078C )
            return 0;
          v8 = v6 + 1;
          sub_1400023E4((unsigned int)(v6 + 1), v4);
          sub_140002B18();
          pulStatus = 0;
          pulProblemNumber = 0;
          DevNode_Status = CM_Get_DevNode_Status(&pulStatus, &pulProblemNumber, DeviceInfoData.DevInst, 0);
          if ( DevNode_Status == 13 )
          {
            pulProblemNumber = 45;
          }
          else if ( DevNode_Status )
          {
            goto LABEL_26;
          }
          Buffer = 0;
          sub_140001854(v23, 0, 518);
          if ( !CM_Get_Device_IDW(DeviceInfoData.DevInst, &Buffer, 0x104u, 0) )
          {
            v10 = sub_140001898(0x960u);
            if ( v10 )
              v11 = sub_140001220(v10);
            else
              v11 = 0;
            *(_QWORD *)(*(_QWORD *)a1 + 8LL * (int)(*(_DWORD *)(a1 + 8))++) = v11;
            *(_DWORD *)(v11 + 2392) = v6;
            *(_DWORD *)v11 = DeviceInfoData.DevInst;
            *(_DWORD *)(v11 + 8) = pulStatus;
            *(_DWORD *)(v11 + 12) = pulProblemNumber;
            lstrcpyW((LPWSTR)(v11 + 16), &Buffer);
            v25 = 104;
            if ( !CM_Get_DevNode_Registry_PropertyW(DeviceInfoData.DevInst, 9u, 0, &String1, &v25, 0) )
            {
              sub_140006D1C(&String1, v11 + 2364);
              if ( !lstrcmpiW(&String1, L"{4d36e978-e325-11ce-bfc1-08002be10318}")
                || !lstrcmpiW(&String1, L"{4D36E96D-E325-11CE-BFC1-08002BE10318}") )
              {
                wsprintfW(v24, L"%s\\%s\\%s", L"System\\CurrentControlSet\\Enum", v11 + 16, L"Device Parameters");
                psz1 = 0;
                *(_QWORD *)pszSrc = 0;
                v17 = 0;
                v18 = 0;
                v19 = 0;
                LODWORD(pulLength) = 14;
                if ( (unsigned int)sub_140006F48(-2147483646, v24, L"PortName", &psz1, pulLength) )
                {
                  if ( !StrCmpNW(&psz1, L"COM", 3) )
                  {
                    lstrcpyW((LPWSTR)(v11 + 536), &psz1);
                    *(_DWORD *)(v11 + 4) = StrToIntW(&pszSrc[2]);
                    QueryDosDeviceW(&psz1, (LPWSTR)(v11 + 564), 0x104u);
                  }
                }
              }
            }
            v25 = 256;
            CM_Get_DevNode_Registry_PropertyW(DeviceInfoData.DevInst, 5u, 0, (PVOID)(v11 + 1596), &v25, 0);
            v25 = 256;
            CM_Get_DevNode_Registry_PropertyW(DeviceInfoData.DevInst, 0x17u, 0, (PVOID)(v11 + 1852), &v25, 0);
            v25 = 256;
            CM_Get_DevNode_Registry_PropertyW(DeviceInfoData.DevInst, 8u, 0, (PVOID)(v11 + 2108), &v25, 0);
            v25 = 512;
            if ( CM_Get_DevNode_Registry_PropertyW(DeviceInfoData.DevInst, 0xDu, 0, (PVOID)(v11 + 1084), &v25, 0) )
            {
              v25 = 512;
              CM_Get_DevNode_Registry_PropertyW(DeviceInfoData.DevInst, 1u, 0, (PVOID)(v11 + 1084), &v25, 0);
            }
            if ( !(unsigned int)sub_1400077D8(DeviceInfoData.DevInst, v11 + 2384) )
              sub_140007738(v11 + 16, v11 + 2384);
          }
LABEL_26:
          ++v6;
        }
      }
    }
  }
  return 0;
}
