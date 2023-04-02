pipeline {
    agent any
    stages {
        stage('run dagger ci') {
            steps {
                sh '''
                    pip install -r ./dagger/requirements.txt
                    python ./dagger/pipeline.py
                '''
            }
        }
    }
}