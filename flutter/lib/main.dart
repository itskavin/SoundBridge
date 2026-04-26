import 'package:flutter/material.dart';

void main() {
  runApp(const SoundBridgeApp());
}

class SoundBridgeApp extends StatelessWidget {
  const SoundBridgeApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SoundBridge',
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  bool _isHosting = false;
  bool _isConnected = false;
  String _status = 'Ready';
  String _serverAddress = '';
  QualityProfile _selectedProfile = QualityProfile.voice;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('SoundBridge'),
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            _buildStatusCard(),
            const SizedBox(height: 16),
            if (!_isConnected) ...[
              _buildModeSelector(),
              const SizedBox(height: 16),
              _buildProfileSelector(),
              const SizedBox(height: 16),
              if (_isHosting) _buildHostSection() else _buildClientSection(),
            ] else ...[
              _buildConnectedCard(),
            ],
            const Spacer(),
            _buildActionButton(),
          ],
        ),
      ),
    );
  }

  Widget _buildStatusCard() {
    final isActive = _isConnected || _isHosting;
    return Card(
      color: isActive
          ? Colors.green.shade50
          : Colors.grey.shade100,
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Row(
          children: [
            Icon(
              isActive ? Icons.wifi : Icons.wifi_off,
              size: 32,
              color: isActive ? Colors.green : Colors.grey,
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    isActive
                        ? (_isConnected ? 'Connected' : 'Hosting')
                        : 'Disconnected',
                    style: const TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  Text(
                    _status,
                    style: TextStyle(
                      color: Colors.grey.shade600,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildModeSelector() {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Padding(
              padding: EdgeInsets.all(8.0),
              child: Text(
                'Mode',
                style: TextStyle(fontWeight: FontWeight.bold),
              ),
            ),
            Row(
              children: [
                Expanded(
                  child: RadioListTile<bool>(
                    title: const Text('Host'),
                    subtitle: const Text('Stream audio to devices'),
                    value: true,
                    groupValue: _isHosting,
                    onChanged: (v) => setState(() => _isHosting = v!),
                  ),
                ),
                Expanded(
                  child: RadioListTile<bool>(
                    title: const Text('Receive'),
                    subtitle: const Text('Play audio from host'),
                    value: false,
                    groupValue: _isHosting,
                    onChanged: (v) => setState(() => _isHosting = v!),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildProfileSelector() {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Quality Profile',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 8),
            SegmentedButton<QualityProfile>(
              segments: const [
                ButtonSegment(
                  value: QualityProfile.voice,
                  label: Text('Voice'),
                  icon: Icon(Icons.mic),
                ),
                ButtonSegment(
                  value: QualityProfile.music,
                  label: Text('Music'),
                  icon: Icon(Icons.music_note),
                ),
                ButtonSegment(
                  value: QualityProfile.ultraLowLatency,
                  label: Text('Ultra Low'),
                  icon: Icon(Icons.speed),
                ),
              ],
              selected: {_selectedProfile},
              onSelectionChanged: (s) => setState(() => _selectedProfile = s.first),
            ),
            const SizedBox(height: 8),
            Text(
              _getProfileDescription(_selectedProfile),
              style: TextStyle(
                color: Colors.grey.shade600,
                fontSize: 12,
              ),
            ),
          ],
        ),
      ),
    );
  }

  String _getProfileDescription(QualityProfile profile) {
    switch (profile) {
      case QualityProfile.voice:
        return 'Best for calls, gaming - 60ms target buffer';
      case QualityProfile.music:
        return 'High quality - 90ms target buffer';
      case QualityProfile.ultraLowLatency:
        return 'Minimum latency - 30ms target buffer';
    }
  }

  Widget _buildHostSection() {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Row(
              children: [
                Icon(Icons.computer),
                SizedBox(width: 8),
                Text(
                  'Host Info',
                  style: TextStyle(fontWeight: FontWeight.bold),
                ),
              ],
            ),
            const SizedBox(height: 12),
            Text(
              'Server running on: 0.0.0.0:7000',
              style: TextStyle(color: Colors.grey.shade700),
            ),
            const SizedBox(height: 8),
            Text(
              'Share this address with devices that should receive audio',
              style: TextStyle(color: Colors.grey.shade500, fontSize: 12),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildClientSection() {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Row(
              children: [
                Icon(Icons.lan),
                SizedBox(width: 8),
                Text(
                  'Connect to Host',
                  style: TextStyle(fontWeight: FontWeight.bold),
                ),
              ],
            ),
            const SizedBox(height: 12),
            TextField(
              decoration: const InputDecoration(
                labelText: 'Server Address',
                hintText: '192.168.1.100:7000',
                border: OutlineInputBorder(),
              ),
              onChanged: (v) => _serverAddress = v,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildConnectedCard() {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text(
              'Connection Details',
              style: TextStyle(fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 12),
            _detailRow('Mode', _isHosting ? 'Hosting' : 'Receiving'),
            _detailRow('Profile', _selectedProfile.name),
            if (!_isHosting) _detailRow('Server', _serverAddress),
          ],
        ),
      ),
    );
  }

  Widget _detailRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(label, style: TextStyle(color: Colors.grey.shade600)),
          Text(value, style: const TextStyle(fontWeight: FontWeight.w500)),
        ],
      ),
    );
  }

  Widget _buildActionButton() {
    final isActive = _isConnected || _isHosting;
    return FilledButton.icon(
      onPressed: isActive ? _disconnect : _connect,
      icon: Icon(isActive ? Icons.stop : Icons.play_arrow),
      label: Text(isActive ? 'Disconnect' : 'Connect'),
      style: FilledButton.styleFrom(
        backgroundColor: isActive ? Colors.red : Colors.green,
        padding: const EdgeInsets.all(16),
      ),
    );
  }

  void _connect() async {
    setState(() {
      _status = 'Connecting...';
    });
    await Future.delayed(const Duration(seconds: 1));
    setState(() {
      _isConnected = true;
      _status = _isHosting ? 'Hosting on port 7000' : 'Connected to $_serverAddress';
    });
  }

  void _disconnect() {
    setState(() {
      _isConnected = false;
      _status = 'Ready';
    });
  }
}

enum QualityProfile {
  voice,
  music,
  ultraLowLatency,
}

extension QualityProfileExt on QualityProfile {
  String get name {
    switch (this) {
      case QualityProfile.voice:
        return 'Voice';
      case QualityProfile.music:
        return 'Music';
      case QualityProfile.ultraLowLatency:
        return 'Ultra Low Latency';
    }
  }
}