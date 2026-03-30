/*
 * func-name: sub_14000641C
 * func-address: 0x14000641c
 * callers: 0x140006850
 * callees: 0x140001854
 */

__int64 sub_14000641C()
{
  unsigned int v1; // ebx
  _EXPLICIT_ACCESS_W pListOfExplicitEntries; // [rsp+50h] [rbp-2B8h] BYREF
  int v3; // [rsp+80h] [rbp-288h]
  int v4; // [rsp+84h] [rbp-284h]
  int v5; // [rsp+88h] [rbp-280h]
  __int64 v6; // [rsp+90h] [rbp-278h]
  int v7; // [rsp+98h] [rbp-270h]
  int v8; // [rsp+9Ch] [rbp-26Ch]
  int v9; // [rsp+A0h] [rbp-268h]
  __int64 *v10; // [rsp+A8h] [rbp-260h]
  int v11; // [rsp+B0h] [rbp-258h]
  int v12; // [rsp+B4h] [rbp-254h]
  int v13; // [rsp+B8h] [rbp-250h]
  __int64 v14; // [rsp+C0h] [rbp-248h]
  int v15; // [rsp+C8h] [rbp-240h]
  int v16; // [rsp+CCh] [rbp-23Ch]
  int v17; // [rsp+D0h] [rbp-238h]
  __int64 *v18; // [rsp+D8h] [rbp-230h]
  int v19; // [rsp+E0h] [rbp-228h]
  int v20; // [rsp+E4h] [rbp-224h]
  int v21; // [rsp+E8h] [rbp-220h]
  __int64 v22; // [rsp+F0h] [rbp-218h]
  int v23; // [rsp+F8h] [rbp-210h]
  int v24; // [rsp+FCh] [rbp-20Ch]
  int v25; // [rsp+100h] [rbp-208h]
  __int64 *v26; // [rsp+108h] [rbp-200h]
  int v27; // [rsp+110h] [rbp-1F8h]
  int v28; // [rsp+114h] [rbp-1F4h]
  int v29; // [rsp+118h] [rbp-1F0h]
  __int64 v30; // [rsp+120h] [rbp-1E8h]
  int v31; // [rsp+128h] [rbp-1E0h]
  int v32; // [rsp+12Ch] [rbp-1DCh]
  int v33; // [rsp+130h] [rbp-1D8h]
  __int64 *v34; // [rsp+138h] [rbp-1D0h]
  char pSecurityDescriptor; // [rsp+140h] [rbp-1C8h] BYREF
  _BYTE v36[47]; // [rsp+141h] [rbp-1C7h] BYREF
  __int64 pSid; // [rsp+170h] [rbp-198h] BYREF
  _BYTE v38[72]; // [rsp+178h] [rbp-190h] BYREF
  __int64 v39; // [rsp+1C0h] [rbp-148h] BYREF
  _BYTE v40[72]; // [rsp+1C8h] [rbp-140h] BYREF
  __int64 v41; // [rsp+210h] [rbp-F8h] BYREF
  _BYTE v42[72]; // [rsp+218h] [rbp-F0h] BYREF
  __int64 v43; // [rsp+260h] [rbp-A8h] BYREF
  _BYTE v44[72]; // [rsp+268h] [rbp-A0h] BYREF
  __int64 v45; // [rsp+2B0h] [rbp-58h] BYREF
  _BYTE v46[72]; // [rsp+2B8h] [rbp-50h] BYREF
  DWORD cbSid; // [rsp+310h] [rbp+8h] BYREF
  PACL NewAcl; // [rsp+318h] [rbp+10h] BYREF

  if ( dword_140016FC0 )
    return 1;
  if ( CoInitializeEx(0, 0) < 0 )
    return 0;
  pSecurityDescriptor = 0;
  sub_140001854(v36, 0, 0x27u);
  pListOfExplicitEntries.grfAccessPermissions = 0;
  sub_140001854(&pListOfExplicitEntries.grfAccessMode, 0, 0xECu);
  NewAcl = 0;
  pSid = 0;
  sub_140001854(v38, 0, 0x40u);
  v45 = 0;
  sub_140001854(v46, 0, 0x40u);
  v39 = 0;
  sub_140001854(v40, 0, 0x40u);
  v43 = 0;
  sub_140001854(v44, 0, 0x40u);
  v41 = 0;
  sub_140001854(v42, 0, 0x40u);
  cbSid = 0;
  v1 = InitializeSecurityDescriptor(&pSecurityDescriptor, 1u);
  if ( v1 )
  {
    cbSid = 72;
    v1 = CreateWellKnownSid(WinBuiltinAdministratorsSid, 0, &pSid, &cbSid);
    if ( v1 )
    {
      cbSid = 72;
      v1 = CreateWellKnownSid(WinLocalServiceSid, 0, &v45, &cbSid);
      if ( v1 )
      {
        cbSid = 72;
        v1 = CreateWellKnownSid(WinNetworkServiceSid, 0, &v39, &cbSid);
        if ( v1 )
        {
          cbSid = 72;
          v1 = CreateWellKnownSid(WinSelfSid, 0, &v43, &cbSid);
          if ( v1 )
          {
            cbSid = 72;
            v1 = CreateWellKnownSid(WinLocalSystemSid, 0, &v41, &cbSid);
            if ( v1 )
            {
              pListOfExplicitEntries.Trustee.ptstrName = (LPWCH)&pSid;
              v10 = &v45;
              pListOfExplicitEntries.grfAccessPermissions = 3;
              v18 = &v39;
              v3 = 3;
              v26 = &v43;
              v11 = 3;
              v19 = 3;
              v27 = 3;
              pListOfExplicitEntries.grfAccessMode = SET_ACCESS;
              v34 = &v41;
              pListOfExplicitEntries.grfInheritance = 0;
              pListOfExplicitEntries.Trustee.pMultipleTrustee = 0;
              pListOfExplicitEntries.Trustee.MultipleTrusteeOperation = NO_MULTIPLE_TRUSTEE;
              pListOfExplicitEntries.Trustee.TrusteeForm = TRUSTEE_IS_SID;
              pListOfExplicitEntries.Trustee.TrusteeType = TRUSTEE_IS_GROUP;
              v4 = 2;
              v5 = 0;
              v6 = 0;
              v7 = 0;
              v8 = 0;
              v9 = 2;
              v12 = 2;
              v13 = 0;
              v14 = 0;
              v15 = 0;
              v16 = 0;
              v17 = 2;
              v20 = 2;
              v21 = 0;
              v22 = 0;
              v23 = 0;
              v24 = 0;
              v25 = 2;
              v28 = 2;
              v29 = 0;
              v30 = 0;
              v31 = 0;
              v32 = 0;
              v33 = 2;
              if ( SetEntriesInAclW(5u, &pListOfExplicitEntries, 0, &NewAcl) || !NewAcl )
                goto LABEL_18;
              v1 = SetSecurityDescriptorOwner(&pSecurityDescriptor, &pSid, 0);
              if ( !v1 )
                goto LABEL_19;
              v1 = SetSecurityDescriptorGroup(&pSecurityDescriptor, &pSid, 0);
              if ( !v1 )
                goto LABEL_19;
              v1 = SetSecurityDescriptorDacl(&pSecurityDescriptor, 1, NewAcl, 0);
              if ( !v1 )
                goto LABEL_19;
              if ( CoInitializeSecurity(&pSecurityDescriptor, -1, 0, 0, 6u, 2u, 0, 0x3000u, 0) < 0 )
              {
LABEL_18:
                v1 = 0;
              }
              else
              {
                v1 = 1;
                dword_140016FC0 = 1;
              }
            }
          }
        }
      }
    }
  }
LABEL_19:
  LocalFree(NewAcl);
  return v1;
}
