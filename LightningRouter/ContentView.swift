//
//  ContentView.swift
//  LightningRouter
//
//  Created by Jackson Coxson on 8/5/25.
//

import SwiftUI
import LightningRouterRS
import AVFoundation

struct ContentView: View {
    @State private var plsEnable = false
    @AppStorage("lastTargetIP") private var targetIP: String = "1.1.1.1"
    @State private var threadKiller: OpaquePointer? = nil
    @State private var connectionStatus: String = "Disconnected"

    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
                .font(.system(size: 40))
                .padding(.top, 20)

            Text("LightningRouter")
                .font(.title2)
                .bold()

            TextField("Enter IP address", text: $targetIP)
                .textFieldStyle(RoundedBorderTextFieldStyle())
                .padding(.horizontal)
                .disableAutocorrection(true)
                .autocapitalization(.none)

            Toggle(isOn: $plsEnable) {
                Text("Enable Tunnel")
            }
            .padding(.horizontal)

            Text("Status: \(connectionStatus)")
                .foregroundColor(plsEnable ? .green : .secondary)
                .font(.footnote)
                .padding(.top, 10)

            Spacer()
        }
        .padding()
        .onChange(of: plsEnable) { newValue in
            print("Tunnel changed: \(newValue), IP: \(targetIP)")

            if threadKiller != nil {
                thread_killer_kill(threadKiller)
                threadKiller = nil
                connectionStatus = "Disconnected"
                UIApplication.shared.isIdleTimerDisabled = false
            }

            if newValue {
                enableBackgroundAudioTrick()
                let cAddress = strdup(targetIP)
                let success = start_simple_udp_proxy(cAddress, 51820, &threadKiller)
                free(cAddress)

                if success {
                    connectionStatus = "Connected"
                    UIApplication.shared.isIdleTimerDisabled = true
                } else {
                    connectionStatus = "Failed to connect"
                    plsEnable = false
                }
            }
        }
    }
}


func enableBackgroundAudioTrick() {
    let session = AVAudioSession.sharedInstance()
    do {
        try session.setCategory(.playback, mode: .default)
        try session.setActive(true)

        let silent = Bundle.main.url(forResource: "silence", withExtension: "mp3")!
        let player = try AVAudioPlayer(contentsOf: silent)
        player.numberOfLoops = -1
        player.volume = 0
        player.play()
    } catch {
        print("Failed to enable background audio: \(error)")
    }
}

#Preview {
    ContentView()
}
