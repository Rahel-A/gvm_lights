package com.gvm.easily.Util;

import android.content.Context;
import android.text.TextUtils;
import android.util.Log;
import com.gvm.easily.Service.UDPClient;

public class ControlUtil {
    private static String flag;

    public static void controlDev(Context context, String str, String str2) {
        String str3 = str + str2;
        if (TextUtils.isEmpty(flag)) {
            flag = str + str2;
        } else if (!flag.equals(str3)) {
            flag = str3;
            Log.d("ControlUtil", "controlCode:" + str2);
            String str4 = "4C5409" + DataUtil.mDevID + DataUtil.mDevType + "5700" + str + "01" + str2;
            String crc16 = CRC16.getCRC16(str4);
            int length = crc16.length();
            if (length == 3) {
                crc16 = "0" + crc16;
            } else if (length == 2) {
                crc16 = "00" + crc16;
            } else if (length == 1) {
                crc16 = "000" + crc16;
            }
            Log.d("ControlUtil", "ControlUtil:" + crc16);
            UDPClient.sendData(context, str4 + crc16);
        }
    }

    public static void responseDev(Context context, String str) {
        Log.d("ControlUtil", "controlCode:" + str);
        String str2 = "4C5409" + DataUtil.mDevID + DataUtil.mDevType + "48" + str;
        String crc16 = CRC16.getCRC16(str2);
        int length = crc16.length();
        if (length == 3) {
            crc16 = "0" + crc16;
        } else if (length == 2) {
            crc16 = "00" + crc16;
        } else if (length == 1) {
            crc16 = "000" + crc16;
        }
        Log.d("ControlUtil", "ControlUtil:" + crc16);
        String str3 = str2 + crc16;
        Log.d("ControlUtil", "回复心跳包:" + str3);
        UDPClient.sendData(context, str3);
    }

    public static void sendPack(Context context) {
        String crc16 = CRC16.getCRC16("4C540900005300000100");
        int length = crc16.length();
        if (length == 3) {
            crc16 = "0" + crc16;
        } else if (length == 2) {
            crc16 = "00" + crc16;
        } else if (length == 1) {
            crc16 = "000" + crc16;
        }
        String str = "4C540900005300000100" + crc16;
        Log.d("ControlUtil", str);
        UDPClient.sendData(context, str);
    }

    public static String getStateOperValue() {
        String str;
        if (DataUtil.mDevType.equals("30")) {
            str = "4C5408" + DataUtil.mDevID + DataUtil.mDevType + "52000005";
        } else {
            str = "4C5408" + DataUtil.mDevID + DataUtil.mDevType + "52000004";
        }
        String crc16 = CRC16.getCRC16(str);
        int length = crc16.length();
        if (length == 3) {
            crc16 = "0" + crc16;
        } else if (length == 2) {
            crc16 = "00" + crc16;
        } else if (length == 1) {
            crc16 = "000" + crc16;
        }
        return str + crc16;
    }
}
