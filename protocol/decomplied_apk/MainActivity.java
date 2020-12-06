package com.gvm.easily.Activity;

import android.app.Activity;
import android.app.Dialog;
import android.content.Intent;
import android.content.IntentFilter;
import android.graphics.drawable.ColorDrawable;
import android.net.Uri;
import android.net.wifi.WifiManager;
import android.os.Bundle;
import android.os.Environment;
import android.os.Handler;
import android.support.p000v4.widget.DrawerLayout;
import android.text.TextUtils;
import android.util.Log;
import android.view.KeyEvent;
import android.view.View;
import android.view.inputmethod.InputMethodManager;
import android.widget.EditText;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.NumberPicker;
import android.widget.ProgressBar;
import android.widget.SeekBar;
import android.widget.TextView;
import butterknife.Bind;
import butterknife.ButterKnife;
import butterknife.OnClick;
import com.example.yideng.loaddialoglibrary.LmiotDialog;
import com.google.gson.Gson;
import com.gvm.easily.App.MyApplication;
import com.gvm.easily.Bean.ConnectBean;
import com.gvm.easily.Bean.PsBean;
import com.gvm.easily.Bean.UdpBean;
import com.gvm.easily.C0630R;
import com.gvm.easily.Receive.NetWorkReceiver;
import com.gvm.easily.Service.UDPClient;
import com.gvm.easily.Service.UDPServer;
import com.gvm.easily.Util.ClickUtils;
import com.gvm.easily.Util.ControlUtil;
import com.gvm.easily.Util.DataUtil;
import com.gvm.easily.Util.DialogUtils;
import com.gvm.easily.Util.JumpActivityUtils;
import com.gvm.easily.Util.ToastUtil;
import com.gvm.easily.View.ColorCircleProgressView;
import com.gvm.easily.View.ColorCircleProgressViewRgb;
import com.gvm.easily.View.TextConfigNumberPicker;
import com.lmiot.tiblebarlibrary.LmiotTitleBar;
import com.makeramen.roundedimageview.RoundedImageView;
import com.zhy.android.percent.support.PercentLayoutHelper;
import com.zhy.android.percent.support.PercentLinearLayout;
import java.io.File;
import java.lang.reflect.Field;
import java.util.Timer;
import java.util.TimerTask;
import org.greenrobot.eventbus.EventBus;
import org.greenrobot.eventbus.Subscribe;
import org.greenrobot.eventbus.ThreadMode;

public class MainActivity extends BaseActivity {
    private long exitTime = 0;
    /* access modifiers changed from: private */
    public int mBrrNum;
    /* access modifiers changed from: private */
    public int mCctNum;
    private boolean mControlFlag;
    /* access modifiers changed from: private */
    public Dialog mDialog;
    private String mFilePath;
    private boolean mFirst = true;
    /* access modifiers changed from: private */
    public String mHexStringRgb;
    @Bind({2131230806})
    TextView mIdChangeDev;
    @Bind({2131230808})
    ColorCircleProgressView mIdColorView01;
    @Bind({2131230809})
    ColorCircleProgressView mIdColorView02;
    @Bind({2131230810})
    ColorCircleProgressViewRgb mIdColorViewRgb;
    @Bind({2131230813})
    DrawerLayout mIdDrawerlayout;
    @Bind({2131230817})
    TextView mIdExit;
    @Bind({2131230819})
    TextView mIdLdMax;
    @Bind({2131230820})
    TextView mIdLdMin;
    @Bind({2131230821})
    TextView mIdLdProgress;
    @Bind({2131230822})
    LinearLayout mIdLed01;
    @Bind({2131230823})
    PercentLinearLayout mIdLed02;
    @Bind({2131230829})
    LinearLayout mIdMenulayout;
    @Bind({2131230830})
    TextConfigNumberPicker mIdNumPicker;
    @Bind({2131230831})
    TextConfigNumberPicker mIdNumPickerRgb;
    @Bind({2131230833})
    ImageView mIdPower01;
    @Bind({2131230834})
    ImageView mIdPower02;
    @Bind({2131230837})
    ProgressBar mIdProbressBar;
    @Bind({2131230838})
    TextView mIdProgress01;
    @Bind({2131230839})
    TextView mIdProgress02;
    @Bind({2131230841})
    SeekBar mIdRgbLd;
    @Bind({2131230842})
    ImageView mIdRgbPower;
    @Bind({2131230843})
    SeekBar mIdRgbSc;
    @Bind({2131230844})
    SeekBar mIdRgbSw;
    @Bind({2131230845})
    ImageView mIdRgbThree;
    @Bind({2131230847})
    TextView mIdScMax;
    @Bind({2131230848})
    TextView mIdScMin;
    @Bind({2131230849})
    TextView mIdScProgress;
    @Bind({2131230852})
    TextView mIdSetting;
    @Bind({2131230854})
    TextView mIdSwMax;
    @Bind({2131230855})
    TextView mIdSwMin;
    @Bind({2131230856})
    TextView mIdSwProgress;
    @Bind({2131230858})
    TextView mIdSxProgress;
    @Bind({2131230860})
    TextView mIdTextCct;
    @Bind({2131230863})
    LmiotTitleBar mIdTitleBar;
    @Bind({2131230881})
    RoundedImageView mImgUserLogo;
    private Intent mIntent;
    /* access modifiers changed from: private */
    public int mLDNum;
    private NetWorkReceiver mNetWorkReceiver;
    /* access modifiers changed from: private */
    public int mPickNum;
    /* access modifiers changed from: private */
    public int mPickNumRGB;
    private boolean mPower = false;
    private boolean mPowerRGB = false;
    /* access modifiers changed from: private */
    public int mProNum = 45;
    /* access modifiers changed from: private */
    public int mSCNum;
    /* access modifiers changed from: private */
    public int mSXNum;
    /* access modifiers changed from: private */
    public int mSwNum;
    private Timer mTimer;
    private TimerTask mTimerTask;

    /* access modifiers changed from: protected */
    public void onCreate(Bundle bundle) {
        super.onCreate(bundle);
        setContentView((int) C0630R.layout.activity_main);
        ButterKnife.bind((Activity) this);
        setTitle();
        setFirst();
        startService(new Intent(this, UDPServer.class));
        this.mIdDrawerlayout.setDrawerLockMode(1);
    }

    private void setFirst() {
        showDev();
        setCirCleListener();
        EventBus.getDefault().register(this);
        registNetWork();
        setPicker();
        setPickerRGB();
        setRGB();
    }

    /* access modifiers changed from: private */
    public void showDev() {
        String jumpDev = DataUtil.getJumpDev(this);
        if (TextUtils.isEmpty(jumpDev)) {
            jumpDev = "light";
        }
        char c = 65535;
        int hashCode = jumpDev.hashCode();
        if (hashCode != 112845) {
            if (hashCode == 102970646 && jumpDev.equals("light")) {
                c = 0;
            }
        } else if (jumpDev.equals("rgb")) {
            c = 1;
        }
        if (c == 0) {
            this.mIdLed01.setVisibility(0);
            this.mIdLed02.setVisibility(8);
        } else if (c == 1) {
            this.mIdLed01.setVisibility(8);
            this.mIdLed02.setVisibility(0);
        }
    }

    private void setRGB() {
        this.mIdRgbSw.setOnSeekBarChangeListener(new SeekBar.OnSeekBarChangeListener() {
            public void onStartTrackingTouch(SeekBar seekBar) {
            }

            public void onStopTrackingTouch(SeekBar seekBar) {
            }

            public void onProgressChanged(SeekBar seekBar, int i, boolean z) {
                if (z) {
                    int unused = MainActivity.this.mSwNum = (DataUtil.mDevSwMin * 100) + (((i * (DataUtil.mDevSwMax - DataUtil.mDevSwMin)) / 100) * 100);
                    MainActivity.this.mIdSwProgress.setText(MainActivity.this.mSwNum + "K");
                    String hexString = Integer.toHexString(MainActivity.this.mSwNum / 100);
                    if (hexString.length() == 1) {
                        hexString = "0" + hexString;
                    }
                    if (TextUtils.isEmpty(MainActivity.this.mHexStringRgb)) {
                        String unused2 = MainActivity.this.mHexStringRgb = hexString;
                    } else if (!MainActivity.this.mHexStringRgb.equals(hexString)) {
                        String unused3 = MainActivity.this.mHexStringRgb = hexString;
                        MainActivity mainActivity = MainActivity.this;
                        mainActivity.controlRGB("03", mainActivity.mHexStringRgb);
                    }
                }
            }
        });
        this.mIdColorViewRgb.setOnProgressListener(new ColorCircleProgressViewRgb.OnProgressListener() {
            public void onScrollingListener(Integer num, boolean z, boolean z2) {
                if (z2) {
                    MainActivity.this.mIdRgbThree.setRotation((float) num.intValue());
                    int unused = MainActivity.this.mSXNum = num.intValue() / 5;
                    MainActivity.this.mIdSxProgress.setText(num + "°");
                    String hexString = Integer.toHexString(MainActivity.this.mSXNum);
                    if (hexString.length() == 1) {
                        hexString = "0" + hexString;
                    }
                    MainActivity.this.controlRGB("04", hexString);
                }
            }
        });
        this.mIdRgbSc.setOnSeekBarChangeListener(new SeekBar.OnSeekBarChangeListener() {
            public void onStartTrackingTouch(SeekBar seekBar) {
            }

            public void onStopTrackingTouch(SeekBar seekBar) {
            }

            public void onProgressChanged(SeekBar seekBar, int i, boolean z) {
                if (z) {
                    int unused = MainActivity.this.mSCNum = i;
                    MainActivity.this.mIdScProgress.setText(MainActivity.this.mSCNum + PercentLayoutHelper.PercentLayoutInfo.BASEMODE.PERCENT);
                    String hexString = Integer.toHexString(MainActivity.this.mSCNum);
                    if (hexString.length() == 1) {
                        hexString = "0" + hexString;
                    }
                    MainActivity.this.controlRGB("05", hexString);
                }
            }
        });
        this.mIdRgbLd.setOnSeekBarChangeListener(new SeekBar.OnSeekBarChangeListener() {
            public void onStartTrackingTouch(SeekBar seekBar) {
            }

            public void onStopTrackingTouch(SeekBar seekBar) {
            }

            public void onProgressChanged(SeekBar seekBar, int i, boolean z) {
                if (z) {
                    int unused = MainActivity.this.mLDNum = i;
                    MainActivity.this.mIdLdProgress.setText(MainActivity.this.mLDNum + PercentLayoutHelper.PercentLayoutInfo.BASEMODE.PERCENT);
                    String hexString = Integer.toHexString(MainActivity.this.mLDNum);
                    if (hexString.length() == 1) {
                        hexString = "0" + hexString;
                    }
                    MainActivity.this.controlRGB("02", hexString);
                }
            }
        });
    }

    /* access modifiers changed from: private */
    public void controlRGB(String str, String str2) {
        ControlUtil.controlDev(this, str, str2);
    }

    private void setPicker() {
        String[] strArr = {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"};
        this.mIdNumPicker.setDisplayedValues(strArr);
        this.mIdNumPicker.setMaxValue(strArr.length - 1);
        this.mIdNumPicker.setMinValue(0);
        this.mIdNumPicker.setValue(this.mPickNum);
        this.mIdNumPicker.setWrapSelectorWheel(false);
        this.mIdNumPicker.setDescendantFocusability(393216);
        setNumberPickerDividerColor(this.mIdNumPicker);
        this.mIdNumPicker.setOnValueChangedListener(new NumberPicker.OnValueChangeListener() {
            public void onValueChange(NumberPicker numberPicker, int i, int i2) {
                int unused = MainActivity.this.mPickNum = i2;
                String hexString = Integer.toHexString(MainActivity.this.mPickNum + 1);
                if (hexString.length() == 1) {
                    hexString = "0" + hexString;
                }
                ControlUtil.controlDev(MainActivity.this, "01", hexString);
            }
        });
    }

    private void setPickerRGB() {
        String[] strArr = {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"};
        this.mIdNumPickerRgb.setDisplayedValues(strArr);
        this.mIdNumPickerRgb.setMaxValue(strArr.length - 1);
        this.mIdNumPickerRgb.setMinValue(0);
        this.mIdNumPickerRgb.setValue(this.mPickNumRGB);
        this.mIdNumPickerRgb.setWrapSelectorWheel(false);
        this.mIdNumPickerRgb.setDescendantFocusability(393216);
        setNumberPickerDividerColor(this.mIdNumPickerRgb);
        this.mIdNumPickerRgb.setOnValueChangedListener(new NumberPicker.OnValueChangeListener() {
            public void onValueChange(NumberPicker numberPicker, int i, int i2) {
                int unused = MainActivity.this.mPickNumRGB = i2;
                String hexString = Integer.toHexString(MainActivity.this.mPickNumRGB + 1);
                if (hexString.length() == 1) {
                    hexString = "0" + hexString;
                }
                MainActivity.this.controlRGB("01", hexString);
            }
        });
    }

    private void setNumberPickerDividerColor(NumberPicker numberPicker) {
        Field[] declaredFields = NumberPicker.class.getDeclaredFields();
        int length = declaredFields.length;
        int i = 0;
        while (i < length) {
            Field field = declaredFields[i];
            if (field.getName().equals("mSelectionDivider")) {
                field.setAccessible(true);
                try {
                    field.set(numberPicker, new ColorDrawable(-1));
                    return;
                } catch (IllegalAccessException e) {
                    e.printStackTrace();
                    return;
                }
            } else {
                i++;
            }
        }
    }

    private void registNetWork() {
        IntentFilter intentFilter = new IntentFilter();
        intentFilter.addAction("android.net.conn.CONNECTIVITY_CHANGE");
        intentFilter.addAction("android.net.wifi.WIFI_STATE_CHANGED");
        this.mNetWorkReceiver = new NetWorkReceiver();
        registerReceiver(this.mNetWorkReceiver, intentFilter);
    }

    /* access modifiers changed from: protected */
    public void onDestroy() {
        super.onDestroy();
        EventBus.getDefault().unregister(this);
        unregisterReceiver(this.mNetWorkReceiver);
        stopTime();
    }

    /* access modifiers changed from: protected */
    public void onPause() {
        super.onPause();
        stopTime();
    }

    /* access modifiers changed from: protected */
    public void onStop() {
        super.onStop();
        stopTime();
    }

    /* access modifiers changed from: protected */
    public void onResume() {
        super.onResume();
        WifiManager wifiManager = (WifiManager) getApplicationContext().getSystemService("wifi");
        if (!wifiManager.isWifiEnabled()) {
            wifiManager.setWifiEnabled(true);
        }
        String str = DataUtil.mDevType;
        char c = 65535;
        int hashCode = str.hashCode();
        int i = 0;
        if (hashCode != 1567) {
            if (hashCode != 1598) {
                if (hashCode == 1629 && str.equals("30")) {
                    c = 2;
                }
            } else if (str.equals("20")) {
                c = 1;
            }
        } else if (str.equals("10")) {
            c = 0;
        }
        if (c == 0) {
            this.mIdColorView02.setViewEnabled(false);
            this.mIdTitleBar.setTitle(!DataUtil.isConnected ? getString(C0630R.string.dev_connecting) : getString(C0630R.string.dev_connected));
        } else if (c == 1) {
            this.mIdColorView02.setViewEnabled(true);
            this.mIdTitleBar.setTitle(!DataUtil.isConnected ? getString(C0630R.string.dev_connecting) : getString(C0630R.string.dev_connected));
        } else if (c == 2) {
            this.mIdColorView02.setViewEnabled(true);
            this.mIdTitleBar.setTitle(!DataUtil.isConnected ? getString(C0630R.string.dev_connecting) : getString(C0630R.string.dev_connected));
        }
        ProgressBar progressBar = this.mIdProbressBar;
        if (DataUtil.isConnected) {
            i = 8;
        }
        progressBar.setVisibility(i);
        startTime();
    }

    private void startTime() {
        this.mTimer = new Timer();
        this.mTimerTask = new TimerTask() {
            public void run() {
                if (!DataUtil.isConnected) {
                    ControlUtil.sendPack(MainActivity.this);
                }
            }
        };
        this.mTimer.schedule(this.mTimerTask, 3000, 3000);
    }

    private void stopTime() {
        try {
            if (this.mTimerTask != null) {
                this.mTimerTask.cancel();
                this.mTimerTask = null;
            }
            if (this.mTimer != null) {
                this.mTimer.cancel();
                this.mTimer = null;
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private void getState() {
        UDPClient.sendData(this, ControlUtil.getStateOperValue());
    }

    private void showUserImg() {
        File file = new File(Environment.getExternalStorageDirectory() + "/GVMIMG", DataUtil.getUserName(this) + "userImg.jpg");
        if (file.exists()) {
            this.mImgUserLogo.setImageURI(Uri.fromFile(file));
            return;
        }
        this.mImgUserLogo.setImageResource(C0630R.C0631drawable.icon);
    }

    private void setCirCleListener() {
        this.mIdColorView01.setOnProgressListener(new ColorCircleProgressView.OnProgressListener() {
            public void onScrollingListener(Integer num, boolean z, boolean z2) {
                if (z2) {
                    MainActivity mainActivity = MainActivity.this;
                    double intValue = (double) num.intValue();
                    Double.isNaN(intValue);
                    int unused = mainActivity.mBrrNum = (int) ((intValue * 0.9d) + 10.0d);
                    TextView textView = MainActivity.this.mIdProgress01;
                    textView.setText(MainActivity.this.mBrrNum + PercentLayoutHelper.PercentLayoutInfo.BASEMODE.PERCENT);
                    MainActivity mainActivity2 = MainActivity.this;
                    mainActivity2.controlDev("BRR", MainActivity.this.mBrrNum + "");
                }
            }
        });
        this.mIdColorView02.setOnProgressListener(new ColorCircleProgressView.OnProgressListener() {
            public void onScrollingListener(Integer num, boolean z, boolean z2) {
                if (z2) {
                    int unused = MainActivity.this.mProNum = DataUtil.mDevSwMax - DataUtil.mDevSwMin;
                    int unused2 = MainActivity.this.mCctNum = (DataUtil.mDevSwMin * 100) + (((num.intValue() * MainActivity.this.mProNum) / 100) * 100);
                    TextView textView = MainActivity.this.mIdProgress02;
                    textView.setText(MainActivity.this.mCctNum + "K");
                    MainActivity mainActivity = MainActivity.this;
                    mainActivity.controlDev("CCT", MainActivity.this.mCctNum + "");
                }
            }
        });
    }

    /* access modifiers changed from: private */
    /* JADX WARNING: Code restructure failed: missing block: B:25:0x0060, code lost:
        if (r14.equals("power") == false) goto L_0x0073;
     */
    /* JADX WARNING: Code restructure failed: missing block: B:44:0x00aa, code lost:
        if (r14.equals("power") != false) goto L_0x00be;
     */
    /* JADX WARNING: Removed duplicated region for block: B:18:0x004b A[ADDED_TO_REGION] */
    /* JADX WARNING: Removed duplicated region for block: B:34:0x0076  */
    /* JADX WARNING: Removed duplicated region for block: B:38:0x0096  */
    /* JADX WARNING: Removed duplicated region for block: B:41:0x00a1  */
    /* JADX WARNING: Removed duplicated region for block: B:48:0x00b5  */
    /* JADX WARNING: Removed duplicated region for block: B:53:0x00c0  */
    /* JADX WARNING: Removed duplicated region for block: B:64:0x010d  */
    /* Code decompiled incorrectly, please refer to instructions dump. */
    public void controlDev(java.lang.String r14, java.lang.String r15) {
        /*
            r13 = this;
            java.lang.String r0 = com.gvm.easily.Util.DataUtil.mDevType
            int r1 = r0.hashCode()
            r2 = 1567(0x61f, float:2.196E-42)
            r3 = 0
            r4 = -1
            r5 = 2
            r6 = 1
            if (r1 == r2) goto L_0x002b
            r2 = 1598(0x63e, float:2.239E-42)
            if (r1 == r2) goto L_0x0021
            r2 = 1629(0x65d, float:2.283E-42)
            if (r1 == r2) goto L_0x0017
            goto L_0x0035
        L_0x0017:
            java.lang.String r1 = "30"
            boolean r0 = r0.equals(r1)
            if (r0 == 0) goto L_0x0035
            r0 = 2
            goto L_0x0036
        L_0x0021:
            java.lang.String r1 = "20"
            boolean r0 = r0.equals(r1)
            if (r0 == 0) goto L_0x0035
            r0 = 1
            goto L_0x0036
        L_0x002b:
            java.lang.String r1 = "10"
            boolean r0 = r0.equals(r1)
            if (r0 == 0) goto L_0x0035
            r0 = 0
            goto L_0x0036
        L_0x0035:
            r0 = -1
        L_0x0036:
            java.lang.String r1 = "03"
            java.lang.String r2 = "00"
            java.lang.String r7 = "power"
            java.lang.String r8 = "CCT"
            java.lang.String r9 = "BRR"
            r10 = 106858757(0x65e8905, float:4.1854225E-35)
            r11 = 66548(0x103f4, float:9.3254E-41)
            r12 = 66050(0x10202, float:9.2556E-41)
            if (r0 == 0) goto L_0x009b
            if (r0 == r6) goto L_0x009b
            if (r0 == r5) goto L_0x0051
            goto L_0x0110
        L_0x0051:
            int r0 = r14.hashCode()
            if (r0 == r12) goto L_0x006b
            if (r0 == r11) goto L_0x0063
            if (r0 == r10) goto L_0x005c
            goto L_0x0073
        L_0x005c:
            boolean r14 = r14.equals(r7)
            if (r14 == 0) goto L_0x0073
            goto L_0x0074
        L_0x0063:
            boolean r14 = r14.equals(r8)
            if (r14 == 0) goto L_0x0073
            r3 = 2
            goto L_0x0074
        L_0x006b:
            boolean r14 = r14.equals(r9)
            if (r14 == 0) goto L_0x0073
            r3 = 1
            goto L_0x0074
        L_0x0073:
            r3 = -1
        L_0x0074:
            if (r3 == 0) goto L_0x0096
            if (r3 == r6) goto L_0x008a
            if (r3 == r5) goto L_0x007c
            goto L_0x0110
        L_0x007c:
            int r14 = java.lang.Integer.parseInt(r15)
            java.lang.Integer.toHexString(r14)
            java.lang.String r14 = "04"
            com.gvm.easily.Util.ControlUtil.controlDev(r13, r14, r15)
            goto L_0x0110
        L_0x008a:
            int r14 = java.lang.Integer.parseInt(r15)
            java.lang.Integer.toHexString(r14)
            com.gvm.easily.Util.ControlUtil.controlDev(r13, r1, r15)
            goto L_0x0110
        L_0x0096:
            com.gvm.easily.Util.ControlUtil.controlDev(r13, r2, r15)
            goto L_0x0110
        L_0x009b:
            int r0 = r14.hashCode()
            if (r0 == r12) goto L_0x00b5
            if (r0 == r11) goto L_0x00ad
            if (r0 == r10) goto L_0x00a6
            goto L_0x00bd
        L_0x00a6:
            boolean r14 = r14.equals(r7)
            if (r14 == 0) goto L_0x00bd
            goto L_0x00be
        L_0x00ad:
            boolean r14 = r14.equals(r8)
            if (r14 == 0) goto L_0x00bd
            r3 = 2
            goto L_0x00be
        L_0x00b5:
            boolean r14 = r14.equals(r9)
            if (r14 == 0) goto L_0x00bd
            r3 = 1
            goto L_0x00be
        L_0x00bd:
            r3 = -1
        L_0x00be:
            if (r3 == 0) goto L_0x010d
            java.lang.String r14 = "0"
            if (r3 == r6) goto L_0x00ea
            if (r3 == r5) goto L_0x00c7
            goto L_0x0110
        L_0x00c7:
            int r15 = java.lang.Integer.parseInt(r15)
            int r15 = r15 / 100
            java.lang.String r15 = java.lang.Integer.toHexString(r15)
            int r0 = r15.length()
            if (r0 != r6) goto L_0x00e6
            java.lang.StringBuilder r0 = new java.lang.StringBuilder
            r0.<init>()
            r0.append(r14)
            r0.append(r15)
            java.lang.String r15 = r0.toString()
        L_0x00e6:
            com.gvm.easily.Util.ControlUtil.controlDev(r13, r1, r15)
            goto L_0x0110
        L_0x00ea:
            int r15 = java.lang.Integer.parseInt(r15)
            java.lang.String r15 = java.lang.Integer.toHexString(r15)
            int r0 = r15.length()
            if (r0 != r6) goto L_0x0107
            java.lang.StringBuilder r0 = new java.lang.StringBuilder
            r0.<init>()
            r0.append(r14)
            r0.append(r15)
            java.lang.String r15 = r0.toString()
        L_0x0107:
            java.lang.String r14 = "02"
            com.gvm.easily.Util.ControlUtil.controlDev(r13, r14, r15)
            goto L_0x0110
        L_0x010d:
            com.gvm.easily.Util.ControlUtil.controlDev(r13, r2, r15)
        L_0x0110:
            return
        */
        throw new UnsupportedOperationException("Method not decompiled: com.gvm.easily.Activity.MainActivity.controlDev(java.lang.String, java.lang.String):void");
    }

    private void setTitle() {
        this.mIdTitleBar.setOnItemClickListener(new LmiotTitleBar.onItemClickListener() {
            public void onMenuClick(View view) {
            }

            public void onTitleClick(View view) {
            }

            public void onBackClick(View view) {
                JumpActivityUtils.JumpToActivity(MainActivity.this, SettingActivity.class, false);
            }
        });
    }

    @OnClick({2131230842, 2131230852, 2131230806, 2131230834, 2131230817})
    public void onViewClicked(View view) {
        ClickUtils.vibrate(this);
        int id = view.getId();
        String str = "01";
        int i = C0630R.C0631drawable.power;
        switch (id) {
            case C0630R.C0632id.id_change_dev:
                this.mIdDrawerlayout.closeDrawer((View) this.mIdMenulayout);
                choseDev();
                return;
            case C0630R.C0632id.id_exit:
                this.mIdDrawerlayout.closeDrawer((View) this.mIdMenulayout);
                MyApplication.getInstance().exit();
                return;
            case C0630R.C0632id.id_power02:
                if (!DataUtil.isConnected) {
                    ToastUtil.ToastMessage(this, getString(C0630R.string.no_con));
                    return;
                }
                this.mPower = !this.mPower;
                ImageView imageView = this.mIdPower02;
                if (!this.mPower) {
                    i = C0630R.C0631drawable.power_off;
                }
                imageView.setImageResource(i);
                if (!this.mPower) {
                    str = "00";
                }
                controlDev("power", str);
                return;
            case C0630R.C0632id.id_rgb_power:
                this.mPowerRGB = !this.mPowerRGB;
                ImageView imageView2 = this.mIdRgbPower;
                if (!this.mPowerRGB) {
                    i = C0630R.C0631drawable.power_off;
                }
                imageView2.setImageResource(i);
                if (!this.mPowerRGB) {
                    str = "00";
                }
                controlRGB("00", str);
                return;
            case C0630R.C0632id.id_setting:
                JumpActivityUtils.JumpToActivity(this, SettingActivity.class, false);
                return;
            default:
                return;
        }
    }

    private void changeWIFI() {
        Dialog dialog = this.mDialog;
        if (dialog != null) {
            dialog.dismiss();
        }
        String ssid = getSSID();
        this.mDialog = DialogUtils.createCenterDialog(this, C0630R.layout.dialog_edit_layout);
        final EditText editText = (EditText) this.mDialog.findViewById(C0630R.C0632id.editText);
        final EditText editText2 = (EditText) this.mDialog.findViewById(C0630R.C0632id.editText_ps);
        TextView textView = (TextView) this.mDialog.findViewById(C0630R.C0632id.tv_title);
        TextView textView2 = (TextView) this.mDialog.findViewById(C0630R.C0632id.tv_sure);
        TextView textView3 = (TextView) this.mDialog.findViewById(C0630R.C0632id.tv_cancle);
        if (!TextUtils.isEmpty(ssid)) {
            editText.setText(ssid);
            editText.setSelection(ssid.length());
        }
        this.mDialog.getWindow().setSoftInputMode(5);
        textView2.setOnClickListener(new View.OnClickListener() {
            public void onClick(View view) {
                String obj = editText.getText().toString();
                if (TextUtils.isEmpty(obj)) {
                    MainActivity mainActivity = MainActivity.this;
                    ToastUtil.ToastMessage(mainActivity, mainActivity.getString(C0630R.string.no_empty));
                    return;
                }
                String obj2 = editText2.getText().toString();
                if (TextUtils.isEmpty(obj2)) {
                    MainActivity mainActivity2 = MainActivity.this;
                    ToastUtil.ToastMessage(mainActivity2, mainActivity2.getString(C0630R.string.wifi_no_empty));
                } else if (obj2.length() < 8) {
                    MainActivity mainActivity3 = MainActivity.this;
                    ToastUtil.ToastMessage(mainActivity3, mainActivity3.getString(C0630R.string.ps_six));
                } else {
                    ((InputMethodManager) MainActivity.this.getApplicationContext().getSystemService("input_method")).hideSoftInputFromWindow(editText.getWindowToken(), 0);
                    try {
                        String json = new Gson().toJson((Object) new PsBean(obj, obj2));
                        Log.d("MainActivity", "json:" + json);
                        LmiotDialog.show(MainActivity.this);
                        UDPClient.sendData(MainActivity.this, json);
                        new Handler().postDelayed(new Runnable() {
                            public void run() {
                                LmiotDialog.hidden();
                            }
                        }, 2000);
                    } catch (Exception e) {
                        e.printStackTrace();
                    }
                    MainActivity.this.mDialog.dismiss();
                }
            }
        });
        textView3.setOnClickListener(new View.OnClickListener() {
            public void onClick(View view) {
                ((InputMethodManager) MainActivity.this.getApplicationContext().getSystemService("input_method")).hideSoftInputFromWindow(editText.getWindowToken(), 0);
                MainActivity.this.mDialog.dismiss();
            }
        });
    }

    private String getSSID() {
        return ((WifiManager) getApplicationContext().getSystemService("wifi")).getConnectionInfo().getSSID().replace("\"", "");
    }

    private void choseDev() {
        this.mDialog = DialogUtils.createBottomDialog(this, C0630R.layout.dialog_picker_dev);
        ((TextView) this.mDialog.findViewById(C0630R.C0632id.id_bt01)).setOnClickListener(new View.OnClickListener() {
            public void onClick(View view) {
                DataUtil.isConnected = false;
                EventBus.getDefault().post(new ConnectBean("data"));
                DataUtil.setJumpDev(MainActivity.this, "light");
                MainActivity.this.showDev();
                MainActivity.this.mDialog.dismiss();
            }
        });
        ((TextView) this.mDialog.findViewById(C0630R.C0632id.id_bt02)).setOnClickListener(new View.OnClickListener() {
            public void onClick(View view) {
                DataUtil.isConnected = false;
                EventBus.getDefault().post(new ConnectBean("data"));
                DataUtil.setJumpDev(MainActivity.this, "rgb");
                MainActivity.this.showDev();
                MainActivity.this.mDialog.dismiss();
            }
        });
        ((TextView) this.mDialog.findViewById(C0630R.C0632id.id_cancel)).setOnClickListener(new View.OnClickListener() {
            public void onClick(View view) {
                MainActivity.this.mDialog.dismiss();
            }
        });
    }

    @Subscribe(threadMode = ThreadMode.MAIN)
    public void onMessageEvent(String str) {
        if (!TextUtils.isEmpty(str) && str.endsWith("no_support")) {
            ToastUtil.ToastMessage(this, getString(C0630R.string.no_sp));
        }
    }

    @Subscribe(threadMode = ThreadMode.MAIN)
    public void onMessageEvent(ConnectBean connectBean) {
        if (!DataUtil.isNetWordConnected) {
            this.mIdProbressBar.setVisibility(0);
            this.mIdTitleBar.setTitle(getString(C0630R.string.dev_connecting));
        } else if (DataUtil.isConnected) {
            if (this.mIdProbressBar.getVisibility() == 0) {
                this.mIdProbressBar.setVisibility(8);
            }
        } else if (this.mIdProbressBar.getVisibility() == 8) {
            this.mIdTitleBar.setTitle(getString(C0630R.string.dev_connecting));
            this.mIdProbressBar.setVisibility(0);
        }
    }

    /* JADX WARNING: Removed duplicated region for block: B:17:0x003b  */
    /* JADX WARNING: Removed duplicated region for block: B:19:0x0044  */
    @android.support.annotation.RequiresApi(api = 24)
    @org.greenrobot.eventbus.Subscribe(threadMode = org.greenrobot.eventbus.ThreadMode.MAIN)
    /* Code decompiled incorrectly, please refer to instructions dump. */
    public void onMessageEvent(com.gvm.easily.Bean.UdpBean r9) {
        /*
            r8 = this;
            java.lang.String r0 = r9.getFlag()
            int r1 = r0.hashCode()
            r2 = 1539(0x603, float:2.157E-42)
            r3 = -1
            r4 = 2
            r5 = 0
            r6 = 1
            if (r1 == r2) goto L_0x002e
            r2 = 1694(0x69e, float:2.374E-42)
            if (r1 == r2) goto L_0x0024
            r2 = 45806640(0x2baf430, float:2.74704E-37)
            if (r1 == r2) goto L_0x001a
            goto L_0x0038
        L_0x001a:
            java.lang.String r1 = "00000"
            boolean r1 = r0.equals(r1)
            if (r1 == 0) goto L_0x0038
            r1 = 2
            goto L_0x0039
        L_0x0024:
            java.lang.String r1 = "53"
            boolean r1 = r0.equals(r1)
            if (r1 == 0) goto L_0x0038
            r1 = 0
            goto L_0x0039
        L_0x002e:
            java.lang.String r1 = "03"
            boolean r1 = r0.equals(r1)
            if (r1 == 0) goto L_0x0038
            r1 = 1
            goto L_0x0039
        L_0x0038:
            r1 = -1
        L_0x0039:
            if (r1 == 0) goto L_0x0044
            if (r1 == r6) goto L_0x003f
            goto L_0x0185
        L_0x003f:
            r8.resloveState(r9)
            goto L_0x0185
        L_0x0044:
            android.widget.ProgressBar r9 = r8.mIdProbressBar
            r1 = 8
            r9.setVisibility(r1)
            java.lang.String r9 = com.gvm.easily.Util.DataUtil.mDevType
            int r1 = r9.hashCode()
            r2 = 1567(0x61f, float:2.196E-42)
            if (r1 == r2) goto L_0x0072
            r2 = 1598(0x63e, float:2.239E-42)
            if (r1 == r2) goto L_0x0068
            r2 = 1629(0x65d, float:2.283E-42)
            if (r1 == r2) goto L_0x005e
            goto L_0x007b
        L_0x005e:
            java.lang.String r1 = "30"
            boolean r9 = r9.equals(r1)
            if (r9 == 0) goto L_0x007b
            r3 = 2
            goto L_0x007b
        L_0x0068:
            java.lang.String r1 = "20"
            boolean r9 = r9.equals(r1)
            if (r9 == 0) goto L_0x007b
            r3 = 1
            goto L_0x007b
        L_0x0072:
            java.lang.String r1 = "10"
            boolean r9 = r9.equals(r1)
            if (r9 == 0) goto L_0x007b
            r3 = 0
        L_0x007b:
            java.lang.String r9 = "light"
            r1 = 2131624121(0x7f0e00b9, float:1.8875413E38)
            r2 = 2131624003(0x7f0e0043, float:1.8875173E38)
            java.lang.String r7 = "K"
            if (r3 == 0) goto L_0x0133
            java.lang.String r5 = "K-"
            if (r3 == r6) goto L_0x00fa
            if (r3 == r4) goto L_0x008f
            goto L_0x0166
        L_0x008f:
            java.lang.String r9 = "rgb"
            com.gvm.easily.Util.DataUtil.setJumpDev(r8, r9)
            r8.showDev()
            com.lmiot.tiblebarlibrary.LmiotTitleBar r9 = r8.mIdTitleBar
            java.lang.String r2 = r8.getString(r2)
            r9.setTitle(r2)
            android.widget.TextView r9 = r8.mIdTextCct
            java.lang.StringBuilder r2 = new java.lang.StringBuilder
            r2.<init>()
            java.lang.String r1 = r8.getString(r1)
            r2.append(r1)
            int r1 = com.gvm.easily.Util.DataUtil.mDevSwMin
            int r1 = r1 * 100
            r2.append(r1)
            r2.append(r5)
            int r1 = com.gvm.easily.Util.DataUtil.mDevSwMax
            int r1 = r1 * 100
            r2.append(r1)
            r2.append(r7)
            java.lang.String r1 = r2.toString()
            r9.setText(r1)
            android.widget.TextView r9 = r8.mIdSwMin
            java.lang.StringBuilder r1 = new java.lang.StringBuilder
            r1.<init>()
            int r2 = com.gvm.easily.Util.DataUtil.mDevSwMin
            int r2 = r2 * 100
            r1.append(r2)
            r1.append(r7)
            java.lang.String r1 = r1.toString()
            r9.setText(r1)
            android.widget.TextView r9 = r8.mIdSwMax
            java.lang.StringBuilder r1 = new java.lang.StringBuilder
            r1.<init>()
            int r2 = com.gvm.easily.Util.DataUtil.mDevSwMax
            int r2 = r2 * 100
            r1.append(r2)
            r1.append(r7)
            java.lang.String r1 = r1.toString()
            r9.setText(r1)
            goto L_0x0166
        L_0x00fa:
            com.gvm.easily.Util.DataUtil.setJumpDev(r8, r9)
            r8.showDev()
            com.lmiot.tiblebarlibrary.LmiotTitleBar r9 = r8.mIdTitleBar
            java.lang.String r2 = r8.getString(r2)
            r9.setTitle(r2)
            android.widget.TextView r9 = r8.mIdTextCct
            java.lang.StringBuilder r2 = new java.lang.StringBuilder
            r2.<init>()
            java.lang.String r1 = r8.getString(r1)
            r2.append(r1)
            int r1 = com.gvm.easily.Util.DataUtil.mDevSwMin
            int r1 = r1 * 100
            r2.append(r1)
            r2.append(r5)
            int r1 = com.gvm.easily.Util.DataUtil.mDevSwMax
            int r1 = r1 * 100
            r2.append(r1)
            r2.append(r7)
            java.lang.String r1 = r2.toString()
            r9.setText(r1)
            goto L_0x0166
        L_0x0133:
            com.gvm.easily.Util.DataUtil.setJumpDev(r8, r9)
            r8.showDev()
            com.gvm.easily.View.ColorCircleProgressView r9 = r8.mIdColorView02
            r9.setViewEnabled(r5)
            com.lmiot.tiblebarlibrary.LmiotTitleBar r9 = r8.mIdTitleBar
            java.lang.String r2 = r8.getString(r2)
            r9.setTitle(r2)
            android.widget.TextView r9 = r8.mIdTextCct
            java.lang.StringBuilder r2 = new java.lang.StringBuilder
            r2.<init>()
            java.lang.String r1 = r8.getString(r1)
            r2.append(r1)
            int r1 = com.gvm.easily.Util.DataUtil.mDevSwMax
            int r1 = r1 * 100
            r2.append(r1)
            r2.append(r7)
            java.lang.String r1 = r2.toString()
            r9.setText(r1)
        L_0x0166:
            android.widget.TextView r9 = r8.mIdProgress02
            java.lang.StringBuilder r1 = new java.lang.StringBuilder
            r1.<init>()
            int r2 = com.gvm.easily.Util.DataUtil.mDevSwMin
            int r2 = r2 * 100
            r1.append(r2)
            r1.append(r7)
            java.lang.String r1 = r1.toString()
            r9.setText(r1)
            int r9 = com.gvm.easily.Util.DataUtil.mDevSwMax
            int r1 = com.gvm.easily.Util.DataUtil.mDevSwMin
            int r9 = r9 - r1
            r8.mProNum = r9
        L_0x0185:
            java.lang.String r9 = "02"
            r0.equals(r9)     // Catch:{ Exception -> 0x018b }
            goto L_0x018f
        L_0x018b:
            r9 = move-exception
            r9.printStackTrace()
        L_0x018f:
            return
        */
        throw new UnsupportedOperationException("Method not decompiled: com.gvm.easily.Activity.MainActivity.onMessageEvent(com.gvm.easily.Bean.UdpBean):void");
    }

    private void resloveState(UdpBean udpBean) {
        try {
            String trim = udpBean.getValue().replace(" ", "").trim();
            String substring = trim.substring(12, trim.length() - 4);
            Log.d("MainActivity", "状态码：" + substring);
            ControlUtil.responseDev(this, substring);
            boolean z = false;
            this.mControlFlag = false;
            if (substring.length() == 8) {
                String substring2 = substring.substring(0, 2);
                this.mPickNum = Integer.parseInt(substring.substring(2, 4), 16);
                String substring3 = substring.substring(4, 6);
                String substring4 = substring.substring(6, 8);
                if (substring2.equals("01")) {
                    z = true;
                }
                this.mPower = z;
                this.mBrrNum = Integer.parseInt(substring3, 16);
                this.mCctNum = Integer.parseInt(substring4, 16) * 100;
                freshState();
            } else if (substring.length() == 12) {
                String substring5 = substring.substring(0, 2);
                this.mPickNumRGB = Integer.parseInt(substring.substring(2, 4), 16);
                String substring6 = substring.substring(4, 6);
                String substring7 = substring.substring(6, 8);
                String substring8 = substring.substring(8, 10);
                String substring9 = substring.substring(10, 12);
                if (substring5.equals("01")) {
                    z = true;
                }
                this.mPowerRGB = z;
                this.mLDNum = Integer.parseInt(substring6, 16);
                this.mSwNum = Integer.parseInt(substring7, 16) * 100;
                Log.d("MainActivity", "mSwNum:" + this.mSwNum);
                this.mSXNum = Integer.parseInt(substring8, 16) * 5;
                this.mSCNum = Integer.parseInt(substring9, 16);
                freshStateRGB();
            }
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private void freshState() {
        this.mIdNumPicker.setValue(this.mPickNum - 1);
        this.mIdPower02.setImageResource(this.mPower ? C0630R.C0631drawable.power : C0630R.C0631drawable.power_off);
        TextView textView = this.mIdProgress01;
        textView.setText(this.mBrrNum + PercentLayoutHelper.PercentLayoutInfo.BASEMODE.PERCENT);
        TextView textView2 = this.mIdProgress02;
        textView2.setText(this.mCctNum + "K");
        this.mIdColorView01.setProgress(((this.mBrrNum + -10) * 10) / 9);
        this.mProNum = DataUtil.mDevSwMax - DataUtil.mDevSwMin;
        this.mIdColorView02.setProgress((this.mCctNum - (DataUtil.mDevSwMin * 100)) / this.mProNum);
    }

    private void freshStateRGB() {
        this.mIdNumPickerRgb.setValue(this.mPickNumRGB - 1);
        this.mIdRgbPower.setImageResource(this.mPowerRGB ? C0630R.C0631drawable.power : C0630R.C0631drawable.power_off);
        TextView textView = this.mIdLdProgress;
        textView.setText(this.mLDNum + PercentLayoutHelper.PercentLayoutInfo.BASEMODE.PERCENT);
        TextView textView2 = this.mIdSwProgress;
        textView2.setText(this.mSwNum + "K");
        TextView textView3 = this.mIdSxProgress;
        textView3.setText(this.mSXNum + "°");
        TextView textView4 = this.mIdScProgress;
        textView4.setText(this.mSCNum + PercentLayoutHelper.PercentLayoutInfo.BASEMODE.PERCENT);
        this.mIdRgbLd.setProgress(this.mLDNum);
        this.mIdRgbSc.setProgress(this.mSCNum);
        this.mProNum = DataUtil.mDevSwMax - DataUtil.mDevSwMin;
        this.mIdRgbSw.setProgress((this.mSwNum - (DataUtil.mDevSwMin * 100)) / this.mProNum);
        this.mIdRgbThree.setRotation((float) this.mSXNum);
    }

    public boolean onKeyDown(int i, KeyEvent keyEvent) {
        if (i != 4) {
            return super.onKeyDown(i, keyEvent);
        }
        exit();
        return false;
    }

    public void exit() {
        if (System.currentTimeMillis() - this.exitTime > 2000) {
            ToastUtil.ToastMessage(this, "再点一次退出！");
            this.exitTime = System.currentTimeMillis();
            return;
        }
        MyApplication.getInstance().exit();
    }
}
