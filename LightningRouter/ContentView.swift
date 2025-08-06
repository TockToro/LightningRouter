//
//  ContentView.swift
//  LightningRouter
//
//  Created by Jackson Coxson on 8/5/25.
//

import SwiftUI
import LightningRouterRS

struct ContentView: View {
    @State private var plsEnable = false
    @State private var targetIP: String = "1.1.1.1"
    @State private var threadKiller: OpaquePointer? = nil
    
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            TextField(
                    "Enter IP",
                    text: $targetIP
                )
                .disableAutocorrection(true)
                .onChange(of: targetIP) {
                    plsEnable = false
                }
            Toggle(isOn: $plsEnable) {
                    Text("Enable Proxy")
                }
            .onChange(of: plsEnable) {
                print("it changed: ", plsEnable, targetIP)
                if threadKiller != nil {
                    thread_killer_kill(threadKiller)
                    threadKiller = nil
                }
                if plsEnable {
                    let cAddress = strdup(targetIP)
                    let _success = start_simple_udp_proxy(cAddress, 51820, &threadKiller)
                }
            }
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
