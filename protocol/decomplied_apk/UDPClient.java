package com.gvm.easily.Service;

import android.content.Context;
import android.net.wifi.WifiManager;
import android.util.Log;
import com.gvm.easily.Util.DataUtil;
import java.net.DatagramPacket;
import java.net.DatagramSocket;
import java.net.InetAddress;

public class UDPClient {
    private static Thread thread;
    private static String value01;

    public static void sendData(final Context context, final String str) {
        if (DataUtil.isNetWordConnected) {
            if (thread != null) {
                thread = null;
            }
            thread = new Thread() {
                public void run() {
                    try {
                        DatagramSocket datagramSocket = new DatagramSocket();
                        byte[] bytes = str.getBytes();
                        String[] split = UDPClient.getIp(context).split("\\.");
                        String str = split[0] + "." + split[1] + "." + split[2] + ".255";
                        Log.d("H", "广播地址：" + str);
                        datagramSocket.send(new DatagramPacket(bytes, bytes.length, InetAddress.getByName(str), 2525));
                        datagramSocket.close();
                        Log.d("UDPClient", "数据发送成功：" + str);
                    } catch (Exception e) {
                        e.printStackTrace();
                    }
                }
            };
            thread.start();
        }
    }

    /* access modifiers changed from: private */
    public static String getIp(Context context) {
        WifiManager wifiManager = (WifiManager) context.getSystemService("wifi");
        if (!wifiManager.isWifiEnabled()) {
            wifiManager.setWifiEnabled(true);
        }
        int ipAddress = wifiManager.getConnectionInfo().getIpAddress();
        return (ipAddress & 255) + "." + ((ipAddress >> 8) & 255) + "." + ((ipAddress >> 16) & 255) + "." + ((ipAddress >> 24) & 255);
    }
}
